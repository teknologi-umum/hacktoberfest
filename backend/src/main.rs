use actix_web::web::Data;
use clap::{clap_app, value_t};
use actix_web::{App, HttpServer, Result};
use std::{io, env, usize};
use std::process::exit;
use std::collections::HashMap;

mod github;
mod handlers;
use crate::handlers::{*};

#[tokio::main]
async fn main() {    
    if let Err(e) = run().await {
        eprintln!("fatal {}", e);
        exit(1);
    }
}

struct RunContext {
    listen_address: String,
    num_workers: usize,
}

async fn run() -> Result<()> {
    if cfg!(debug_assertions) { color_backtrace::install() }

    let app = clap_app!(hacktoberfestd => 
        (version: "")
        (about: "Hacktoberfest serverd")
        (@arg addr: --addr +takes_value "Listen address for HTTP server")
        (@arg wrk: --wrk +takes_value "Number of HTTP server workers")
    ).get_matches();
    
    let falback_laddr = env::var("LISTEN_ADDR").unwrap_or("127.0.0.1:8080".into());
    let fallback_num_wrk_str = env::var("NUM_WORKERS").unwrap_or("1".into());
    let mut fallback_num_wrk = 1;
    if let Ok(num_wrk) = fallback_num_wrk_str.parse::<usize>() {
        fallback_num_wrk = num_wrk;
    }

    let laddr = app.value_of("addr").unwrap_or(&falback_laddr[..]);
    let num_wrk = value_t!(app, "wrk", usize).unwrap_or(fallback_num_wrk);
    
    let env = RunContext {
        listen_address: String::from(laddr),
        num_workers: num_wrk,
    };

    if let Err(e) = run_server(env).await {
        return Err(e.into())
    }
    Ok(())
}

async fn run_server(env: RunContext) -> Result<(), io::Error> {
    println!("run server {}", env.listen_address);

    let global_map = Data::new(HashMap::<String, String>::new());
    HttpServer::new(move || {
        App::new()
            .app_data(global_map.clone())
            .service(healthcheck::Handler())
            .service(repositories::Handler())
    })
        .bind(env.listen_address)?
        .workers(env.num_workers)
        .run()
        .await
}
