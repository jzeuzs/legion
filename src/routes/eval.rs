use std::process::Output;

use anyhow::anyhow;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use nanoid::nanoid;
use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};
use tokio::process::Command;
use tokio::time::{sleep, Duration};
use tracing::{error, info, warn};

use crate::docker::{container_exists, exec, start_container};
use crate::{Config, Result};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Eval {
    language: String,
    code: String,
    input: Option<String>,
    args: Option<Vec<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EvalResult {
    stdout: String,
    stderr: String,
    status: EvalStatus,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EvalStatus {
    success: bool,
    code: Option<i32>,
}

pub async fn eval(State(config): State<Config>, Json(payload): Json<Eval>) -> Result<Response> {
    if !config.language.enabled.contains(&payload.language) {
        return Ok((
            StatusCode::NOT_FOUND,
            format!("The language {} is not enabled or does not exist.", payload.language),
        )
            .into_response());
    }

    let id = nanoid!();
    let container_present = container_exists(&payload.language).await?;

    if !container_present {
        if config.prepare_containers {
            warn!(
                "[{}] Container legion-{} is not present. Starting a new container.",
                id.yellow(),
                payload.language
            );

            start_container(&payload.language, &config.language).await?;
        } else {
            error!("[{}] Container legion-{} is not present.", id.yellow(), payload.language);

            return Err(
                anyhow!(format!("Container legion-{} does not exist.", payload.language)).into()
            );
        }
    }

    exec(&[
        "exec",
        &format!("legion-{}", payload.language),
        "mkdir",
        "-p",
        &format!("eval/{}", id),
    ])
    .await?;

    exec(&[
        "exec",
        &format!("legion-{}", payload.language),
        "chmod",
        "777",
        &format!("eval/{}", id),
    ])
    .await?;

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
            _ = sleep(Duration::from_secs_f64(config.language.timeout)) => {
                exec(&["kill", &format!("legion-{}", payload.language)]).await?;
                start_container(&payload.language, &config.language).await?;

                return Ok((StatusCode::GATEWAY_TIMEOUT, "Eval timed out.".to_string()).into_response())
            },
            output = _eval(&payload.language, &payload.code, payload.input.as_deref(), payload.args.as_deref(), &id) => {
                match output {
                    Ok(output)  => {
                        if success || output.status.success() {
                            success = true;
                            break output;
                        }

                        if !success && config.language.retries == times_failed {
                            break output;
                        }

                        success = false;
                        times_failed += 1;
                    },
                    Err(err) => {
                        success = false;
                        times_failed += 1;

                        if !success && config.language.retries == times_failed {
                            return Err(err)
                        }
                    }
                }
            }
        };
    };

    exec(&["exec", &format!("legion-{}", payload.language), "rm", "-rf", &format!("eval/{}", id)])
        .await?;

    info!(
        "[{}] Finished eval in container {}.",
        id.yellow(),
        format!("legion-{}", payload.language).underline().bold()
    );

    let response = EvalResult {
        stdout: String::from_utf8_lossy(&output.stdout).to_string(),
        stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        status: EvalStatus {
            success: output.status.success() || success,
            code: output.status.code(),
        },
    };

    Ok(Json(response).into_response())
}

async fn _eval(
    language: &str,
    code: &str,
    input: Option<&str>,
    args: Option<&[String]>,
    uid: &str,
) -> Result<Output> {
    let mut cmd = Command::new("docker");

    cmd.args([
        "exec",
        "-u1001:1001",
        "-i",
        &format!("-w/tmp/eval/{}", uid),
        &format!("legion-{}", language),
        "/bin/sh",
        "/var/run/run.sh",
        code,
        input.unwrap_or_default(),
    ]);

    let args = args.unwrap_or_default();

    if !args.is_empty() {
        cmd.args(args);
    }

    Ok(cmd.output().await?)
}

#[cfg(test)]
mod test {
    use std::sync::Arc;

    use axum::body::Body;
    use axum::http::{header, Method, Request, StatusCode};
    use http_body_util::BodyExt;
    use tokio::fs;
    use tower::ServiceExt;

    use super::{Eval, EvalResult};
    use crate::app;
    use crate::config::{Config, Language};
    use crate::docker::{build_images, exec, prepare_containers};

    macro_rules! gen_test {
        ($($name:ident, $filename:expr;)+) => {
            $(
                #[tokio::test]
                async fn $name() {
                    let config = Arc::new(Config {
                        prepare_containers: true,
                        language: Language {
                            timeout: 30.0,
                            enabled: vec![stringify!($name).to_owned()],
                            ..Language::default()
                        },
                        ..Config::default()
                    });

                    let app = app(config);

                    if option_env!("LEGION_TEST_BUILD").unwrap_or("0") != "0" {
                        build_images(&[stringify!($name).to_owned()], true).await.expect("Failed building images");
                    }

                    prepare_containers(&[stringify!($name).to_owned()], &Language {
                        timeout: 30.0,
                        enabled: vec![stringify!($name).to_owned()],
                        ..Language::default()
                    })
                    .await
                    .expect("Failed preparing containers.");

                    let response = app
                        .oneshot(
                            Request::builder()
                                .method(Method::POST)
                                .uri("/eval")
                                .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                                .body(Body::from(
                                    serde_json::to_string(&Eval {
                                        language: stringify!($name).to_owned(),
                                        code: fs::read_to_string(format!("test-programs/{}", $filename))
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
                    exec(&["kill", &format!("legion-{}", stringify!($name))]).await.expect("Failed killing container");
                    exec(&["rm", "-f", "-l", &format!("legion-{}", stringify!($name))]).await.expect("Failed deleting container");
                }
            )*
        }
    }

    gen_test! {
        assembly_as, "assembly_as.s";
        assembly_fasm, "assembly_fasm.s";
        assembly_gcc, "assembly_gcc.s";
        assembly_jwasm, "assembly_jwasm.s";
        assembly_nasm, "assembly_nasm.s";
        bash, "bash.sh";
        befunge, "befunge.b93";
        brainfuck, "brainfuck.bf";
        bun, "bun.js";
        c, "c.c";
        cpp, "cpp.cc";
        crystal, "crystal.cr";
        csharp, "csharp.cs";
        deno, "deno.ts";
        elixir, "elixir.exs";
        erlang, "erlang.erl";
        fsharp, "fsharp.fs";
        haskell, "haskell.hs";
        java, "java.java";
        javascript, "javascript.js";
        julia, "julia.jl";
        lolcode, "lolcode.lol";
        lua, "lua.lua";
        objective_c, "objective_c.m";
        perl, "perl.pl";
        php, "php.php";
        python, "python.py";
        ruby, "ruby.rb";
        rust, "rust.rs";
        shakespeare, "shakespeare.spl";
        spim, "spim.s";
        sqlite, "sqlite.sql";
        typescript, "typescript.ts";
        zig, "zig.zig";
    }
}
