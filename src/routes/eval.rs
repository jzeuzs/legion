use std::process::Output;

use anyhow::Result;
use owo_colors::OwoColorize;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::tokio::process::Command;
use rocket::tokio::time::{sleep, Duration};
use rocket::{tokio, State};
use snowflake::ProcessUniqueId;

use crate::docker::{container_exists, exec, start_container};
use crate::{Cache, Config};

#[derive(Clone, Deserialize, PartialEq, Eq, Hash, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Payload {
    language: String,
    code: String,
    #[serde(default)]
    input: String,
    #[serde(default)]
    args: Vec<String>,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Response {
    stdout: String,
    stderr: String,
    status: EvalStatus,
}

#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct EvalStatus {
    success: bool,
    code: Option<i32>,
}

#[post("/eval", format = "json", data = "<payload>")]
pub async fn eval(
    payload: Json<Payload>,
    config: &State<Config>,
    cache: &State<Cache>,
) -> Result<Json<Response>, Custom<String>> {
    if !config.language.enabled.contains(&payload.language) {
        return Err(Custom(
            Status::NotFound,
            format!("The language {} does not exist", payload.language),
        ));
    }

    let uid = ProcessUniqueId::new().to_string();
    let container_present = container_exists(&payload.language)
        .map_err(|err| Custom(Status::InternalServerError, err.to_string()))?;

    if !container_present {
        if config.prepare_containers {
            warn!(
                "[{}] Container legion-{} not present. Starting a new container.",
                uid.yellow(),
                payload.language
            );

            start_container(&payload.language, &config.language)
                .map_err(|err| Custom(Status::InternalServerError, err.to_string()))?;
        } else {
            error!(
                "[{}] Container legion-{} not present. Starting a new container.",
                uid.yellow(),
                payload.language
            );

            return Err(Custom(
                Status::InternalServerError,
                format!("Container legion-{} does not exist.", payload.language),
            ));
        }
    }

    if config.cache.enabled {
        match cache.get(&payload.0) {
            Some(cached) => {
                info!("[{}] Cache hit!", uid.yellow());

                return Ok(Json(cached));
            },
            None => {
                info!("[{}] Cache miss...", uid.yellow());
            },
        }
    }

    exec(&[
        "exec",
        &format!("legion-{}", payload.language),
        "mkdir",
        "-p",
        &format!("eval/{}", uid),
    ])
    .map_err(|err| Custom(Status::InternalServerError, err.to_string()))?;

    exec(&[
        "exec",
        &format!("legion-{}", payload.language),
        "chmod",
        "777",
        &format!("eval/{}", uid),
    ])
    .map_err(|err| Custom(Status::InternalServerError, err.to_string()))?;

    info!(
        "[{}] Eval in container {}...",
        uid.yellow(),
        format!("legion-{}", payload.language).underline().bold()
    );

    let mut times_failed: u8 = 0;
    let mut success = false;

    #[allow(unused_assignments)]
    let output = loop {
        tokio::select! {
            _ = sleep(Duration::from_secs_f64(config.language.timeout)) => {
                exec(&["kill", &format!("legion-{}", payload.language)]).map_err(|err| Custom(Status::InternalServerError, err.to_string()))?;
                start_container(&payload.language, &config.language).map_err(|err| Custom(Status::InternalServerError, err.to_string()))?;

                return Err(Custom(Status::GatewayTimeout, "Eval timed out".to_owned()));
            },
            output = _eval(&payload.language, &payload.code, &payload.input, &payload.args, &uid) => {
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
                            return Err(Custom(Status::InternalServerError, err.to_string()))
                        }
                    }
                }
            }
        };
    };

    exec(&["exec", &format!("legion-{}", payload.language), "rm", "-rf", &format!("eval/{}", uid)])
        .map_err(|err| Custom(Status::InternalServerError, err.to_string()))?;

    info!(
        "[{}] Finished eval in container {}.",
        uid.yellow(),
        format!("legion-{}", payload.language).underline().bold()
    );

    let response = Response {
        stdout: String::from_utf8_lossy(&output.stdout).to_string(),
        stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        status: EvalStatus {
            success: output.status.success() || success,
            code: output.status.code(),
        },
    };

    if config.cache.enabled {
        cache.insert(payload.0, response.clone()).await;
    }

    Ok(Json(response))
}

async fn _eval(
    language: &str,
    code: &str,
    input: &str,
    args: &[String],
    uid: &str,
) -> Result<Output> {
    let mut cmd = Command::new("docker");

    cmd.args(&[
        "exec",
        "-u1001:1001",
        "-i",
        &format!("-w/tmp/eval/{}", uid),
        &format!("legion-{}", language),
        "/bin/sh",
        "/var/run/run.sh",
        code,
        input,
    ]);

    if !args.is_empty() {
        cmd.args(args);
    }

    Ok(cmd.output().await?)
}

#[cfg(test)]
mod test {
    use std::fs;
    use std::sync::Arc;

    use rocket::http::{ContentType, Status};
    use rocket::local::blocking::Client;
    use rocket::serde::json::{from_str, to_string};
    use rocket::{Build, Rocket};

    use super::{eval, exec, Cache, Payload, Response};
    use crate::config::{Config, Language};
    use crate::docker::{build_images, prepare_containers};

    macro_rules! gen_test {
        ($($name:ident, $filename:expr;)+) => {
            $(
                #[test]
                fn $name() {
                    #[allow(clippy::no_effect_underscore_binding)]
                    fn init_server() -> Rocket<Build> {
                        rocket::build()
                            .mount("/", routes![eval])
                            .manage(Arc::new(Config {
                                prepare_containers: true,
                                language: Language {
                                    timeout: 30.0,
                                    enabled: vec![stringify!($name).to_owned()],
                                    ..Language::default()
                                },
                                ..Config::default()
                            }))
                            .manage(Cache::new(100))
                    }

                    let client = Client::tracked(init_server()).unwrap();

                    build_images(&[stringify!($name).to_owned()], true).expect("Failed building images");
                    prepare_containers(&[stringify!($name).to_owned()], &Language {
                        timeout: 30.0,
                        enabled: vec![stringify!($name).to_owned()],
                        ..Language::default()
                    })
                    .expect("Failed preparing containers");

                    let res =
                        client
                            .post("/eval")
                            .header(ContentType::JSON)
                            .body(
                                to_string(&Payload {
                                    language: stringify!($name).to_owned(),
                                    code: fs::read_to_string(format!("test-programs/{}", $filename))
                                        .expect("Test program not found"),
                                    args: vec![],
                                    input: String::new(),
                                })
                                .expect("Failed converting to json string"),
                            )
                            .dispatch();

                    assert_eq!(res.status(), Status::Ok);
                    assert_eq!(res.content_type(), Some(ContentType::JSON));

                    let body: Response = from_str(&res.into_string().expect("Body empty")).expect("Invalid body");

                    assert_eq!(body.stdout.trim(), "Hello, World!", "{}", body.stderr.trim());

                    // Removing containers as they can cause unwanted clutter in the user's device
                    exec(&["kill", &format!("legion-{}", stringify!($name))]).expect("Failed killing container");
                    exec(&["rm", "-f", "-l", &format!("legion-{}", stringify!($name))]).expect("Failed deleting container");
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
        go, "go.go";
        haskell, "haskell.hs";
        java, "java.java";
        javascript, "javascript.js";
        julia, "julia.jl";
        lolcode, "lolcode.lol";
        lua, "lua.lua";
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
