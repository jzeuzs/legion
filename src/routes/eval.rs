use std::sync::Arc;

use anyhow::anyhow;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use bollard::container::{LogOutput, RemoveContainerOptions};
use bollard::exec::{CreateExecOptions, StartExecResults};
use futures_util::StreamExt;
use nanoid::nanoid;
use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};
use tokio::time::{sleep, Duration};
use tracing::{error, info, warn};
use utoipa::ToSchema;

use crate::docker::{container_exists, start_container};
use crate::{AppState, Result};

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct Eval {
    #[schema(example = "javascript")]
    language: String,
    #[schema(example = "console.log('Hello, World!');")]
    code: String,
    input: Option<String>,
    args: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct EvalResult {
    #[schema(example = "Hello, World!")]
    stdout: String,
    stderr: String,
    stdin: String,
    status: EvalStatus,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct EvalStatus {
    #[schema(example = true)]
    success: bool,
    #[schema(example = 0)]
    code: Option<i32>,
}

#[utoipa::path(
    post,
    path = "/api/eval",
    request_body = Eval,
    responses(
        (status = 200, body = EvalResult),
        (status = 500, description = "Server error."),
        (status = 404, description = "Language is not enabled or does not exist."),
        (status = 408, description = "Execution timeout.")
    )
)]
pub async fn eval(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Eval>,
) -> Result<Response> {
    if !state.config.language.enabled.contains(&payload.language) {
        return Ok((
            StatusCode::NOT_FOUND,
            format!("The language {} is not enabled or does not exist.", payload.language),
        )
            .into_response());
    }

    let id = nanoid!();
    let container_present = container_exists(&state, &payload.language).await?;

    if !container_present {
        if state.config.prepare_containers {
            warn!(
                "[{}] Container legion-{} is not present. Starting a new container.",
                id.yellow(),
                payload.language
            );

            start_container(&state, &payload.language).await?;
        } else {
            error!("[{}] Container legion-{} is not present.", id.yellow(), payload.language);

            return Err(
                anyhow!(format!("Container legion-{} does not exist.", payload.language)).into()
            );
        }
    }

    // Create dir for execution
    let create_exec_options = CreateExecOptions {
        cmd: Some(vec!["mkdir".to_string(), "-p".to_string(), format!("eval/{}", id)]),
        ..Default::default()
    };

    let create_exec = state
        .docker
        .create_exec(&format!("legion-{}", payload.language), create_exec_options)
        .await?;

    state.docker.start_exec(&create_exec.id, None).await?;

    // Set perms
    let create_exec_options = CreateExecOptions {
        cmd: Some(vec!["chmod".to_string(), "777".to_string(), format!("eval/{}", id)]),
        ..Default::default()
    };

    let create_exec = state
        .docker
        .create_exec(&format!("legion-{}", payload.language), create_exec_options)
        .await?;

    state.docker.start_exec(&create_exec.id, None).await?;

    info!(
        "[{}] Eval in container {}...",
        id.yellow(),
        format!("legion-{}", payload.language).underline().bold()
    );

    let mut times_failed: u8 = 0;
    let mut success = false;

    #[allow(clippy::ignored_unit_patterns, unused_assignments)]
    let output = loop {
        tokio::select! {
            _ = sleep(Duration::from_secs_f64(state.config.language.timeout)) => {
                let remove_container_options = RemoveContainerOptions {
                    force: true,
                    ..Default::default()
                };

                state
                    .docker
                    .remove_container(&format!("legion-{}", payload.language), Some(remove_container_options))
                    .await?;

                start_container(&state, &payload.language).await?;

                return Ok((StatusCode::REQUEST_TIMEOUT, "Eval timed out.".to_string()).into_response())
            },
            output = _eval(&payload.language, &payload.code, payload.input.as_deref(), payload.args.as_deref(), &id, &state) => {
                match output {
                    Ok(output)  => {
                        if success || output.status.success {
                            success = true;
                            break output;
                        }

                        if !success && state.config.language.retries == times_failed {
                            break output;
                        }

                        success = false;
                        times_failed += 1;
                    },
                    Err(err) => {
                        success = false;
                        times_failed += 1;

                        if !success && state.config.language.retries == times_failed {
                            return Err(err)
                        }
                    }
                }
            }
        };
    };

    // Remove execution dir
    let create_exec_options = CreateExecOptions {
        cmd: Some(vec!["rm".to_string(), "-rf".to_string(), format!("eval/{}", id)]),
        ..Default::default()
    };

    let create_exec = state
        .docker
        .create_exec(&format!("legion-{}", payload.language), create_exec_options)
        .await?;

    state.docker.start_exec(&create_exec.id, None).await?;

    info!(
        "[{}] Finished eval in container {}.",
        id.yellow(),
        format!("legion-{}", payload.language).underline().bold()
    );

    Ok(Json(output).into_response())
}

async fn _eval(
    language: &str,
    code: &str,
    input: Option<&str>,
    args: Option<&[String]>,
    uid: &str,
    state: &Arc<AppState>,
) -> Result<EvalResult> {
    let mut cmd = vec![
        "nice".to_string(),
        "prlimit".to_string(),
        format!("--nproc={}", state.config.language.max_process_count),
        format!("--nofile={}", state.config.language.max_open_files),
        format!("--fsize={}", state.config.language.max_file_size),
        "/bin/sh".to_string(),
        "/var/run/run.sh".to_string(),
        code.to_string(),
        input.unwrap_or_default().to_string(),
    ];

    if let Some(args) = args {
        cmd.extend(args.iter().cloned());
    }

    let create_exec_options = CreateExecOptions {
        cmd: Some(cmd),
        user: Some("1001:1001".to_string()),
        working_dir: Some(format!("/tmp/eval/{}", uid)),
        attach_stderr: Some(true),
        attach_stdout: Some(true),
        ..Default::default()
    };

    let create_exec =
        state.docker.create_exec(&format!("legion-{}", language), create_exec_options).await?;

    let StartExecResults::Attached {
        mut output, ..
    } = state.docker.start_exec(&create_exec.id, None).await?
    else {
        unreachable!()
    };

    let mut stdout = Vec::new();
    let mut stderr = String::new();
    let mut stdin = String::new();

    while let Some(Ok(log)) = output.next().await {
        match log {
            LogOutput::StdOut {
                message,
            } => stdout.push(String::from_utf8_lossy(&message).to_string()),
            LogOutput::StdErr {
                message,
            } => stderr.push_str(String::from_utf8_lossy(&message).as_ref()),
            LogOutput::StdIn {
                message,
            } => stdin.push_str(String::from_utf8_lossy(&message).as_ref()),
            LogOutput::Console {
                ..
            } => (),
        }
    }

    let code = stdout.pop().map(|code| code.parse::<i32>()).transpose()?;
    let result = EvalResult {
        stderr,
        stdin,
        stdout: stdout.join(""),
        status: EvalStatus {
            success: code.is_some_and(|code| code == 0),
            code,
        },
    };

    Ok(result)
}

#[cfg(test)]
mod test {
    use std::sync::Arc;

    use axum::body::Body;
    use axum::http::{header, Method, Request, StatusCode};
    use bollard::container::RemoveContainerOptions;
    use bollard::Docker;
    use http_body_util::BodyExt;
    use nanoid::nanoid;
    use paste::paste;
    use tokio::fs;
    use tower::ServiceExt;

    use super::{Eval, EvalResult};
    use crate::config::{Config, Language};
    use crate::docker::{build_images, prepare_containers};
    use crate::{app, AppState};

    async fn remove_container(state: &Arc<AppState>, language: &str) {
        let remove_container_options = RemoveContainerOptions {
            force: true,
            ..Default::default()
        };

        state
            .docker
            .remove_container(&format!("legion-{}", language), Some(remove_container_options))
            .await
            .unwrap();
    }

    macro_rules! gen_test {
        ($($name:ident, $ext:expr;)+) => {
            $(
                paste! {
                    #[tokio::test]
                    async fn [<$name _hello_world>]() {
                        let config = Config {
                            prepare_containers: true,
                            language: Language {
                                timeout: 30.0,
                                enabled: vec![stringify!($name).to_owned()],
                                ..Language::default()
                            },
                            ..Config::default()
                        };

                        let docker = Docker::connect_with_socket_defaults().unwrap();

                        let state = Arc::new(AppState {
                            config,
                            docker,
                        });

                        let app = app(state.clone());

                        if option_env!("LEGION_TEST_BUILD").unwrap_or("0") != "0" {
                            build_images(&state).await.expect("Failed building images");
                        }

                        prepare_containers(&state).await.expect("Failed preparing containers.");

                        let response = app
                            .oneshot(
                                Request::builder()
                                    .method(Method::POST)
                                    .uri("/api/eval")
                                    .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                                    .body(Body::from(
                                        serde_json::to_string(&Eval {
                                            language: stringify!($name).to_owned(),
                                            code: fs::read_to_string(format!("test-programs/{}/hello-world{}", stringify!($name).to_owned(), $ext))
                                                .await.expect("Test program not found"),
                                            args: Some(vec![]),
                                            input: Some(String::new()),
                                        })
                                        .expect("Failed converting to json string")
                                    ))
                                    .unwrap()
                            )
                            .await
                            .unwrap();

                        assert_eq!(response.status(), StatusCode::OK);

                        let body = response.into_body().collect().await.unwrap().to_bytes();
                        let body: EvalResult = serde_json::from_slice(&body).unwrap();

                        assert!(body.stdout.contains("Hello, World!"), "stderr: {} \n\nstdout: {}", body.stderr.trim(), body.stdout.trim());

                        // Removing containers as they can cause unwanted clutter in the user's device
                        remove_container(&state, stringify!($name)).await;
                    }

                    #[tokio::test]
                    async fn [<$name _input>]() {
                        let config = Config {
                            prepare_containers: true,
                            language: Language {
                                timeout: 30.0,
                                enabled: vec![stringify!($name).to_owned()],
                                ..Language::default()
                            },
                            ..Config::default()
                        };

                        let docker = Docker::connect_with_socket_defaults().unwrap();

                        let state = Arc::new(AppState {
                            config,
                            docker,
                        });

                        let app = app(state.clone());

                        if option_env!("LEGION_TEST_BUILD").unwrap_or("0") != "0" {
                            build_images(&state).await.expect("Failed building images");
                        }

                        prepare_containers(&state).await.expect("Failed preparing containers.");

                        let input = nanoid!();
                        let response = app
                            .oneshot(
                                Request::builder()
                                    .method(Method::POST)
                                    .uri("/api/eval")
                                    .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                                    .body(Body::from(
                                        serde_json::to_string(&Eval {
                                            language: stringify!($name).to_owned(),
                                            code: fs::read_to_string(format!("test-programs/{}/input{}", stringify!($name).to_owned(), $ext))
                                                .await.expect("Test program not found"),
                                            args: Some(vec![]),
                                            input: Some(input.clone()),
                                        })
                                        .expect("Failed converting to json string")
                                    ))
                                    .unwrap()
                            )
                            .await
                            .unwrap();

                        assert_eq!(response.status(), StatusCode::OK);

                        let body = response.into_body().collect().await.unwrap().to_bytes();
                        let body: EvalResult = serde_json::from_slice(&body).unwrap();

                        assert!(body.stdout.contains(&input), "stderr: {} \n\nstdout: {}", body.stderr.trim(), body.stdout.trim());

                        // Removing containers as they can cause unwanted clutter in the user's device
                        remove_container(&state, stringify!($name)).await;
                    }
                }
            )*
        }
    }

    gen_test! {
        assembly_as, ".s";
        bash, ".sh";
        befunge, ".b93";
        brainfuck, ".bf";
        bun, ".js";
        c, ".c";
        cpp, ".cc";
        crystal, ".cr";
        csharp, ".cs";
        deno, ".ts";
        fsharp, ".fs";
        haskell, ".hs";
        java, ".java";
        javascript, ".js";
        julia, ".jl";
        lolcode, ".lol";
        lua, ".lua";
        perl, ".pl";
        php, ".php";
        python, ".py";
        ruby, ".rb";
        rust, ".rs";
        spim, ".s";
        typescript, ".ts";
    }
}
