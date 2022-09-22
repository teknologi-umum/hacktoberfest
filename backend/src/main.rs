use clap::{clap_app, value_t};
use actix_web::{App, get, HttpResponse, HttpServer, web, Result};
use core::num;
use std::{io, env, usize};
use std::collections::HashMap;
use std::process::exit;

mod github;

#[get("/health")]
async fn healthcheck() -> Result<HttpResponse> {
    let mut resp: HashMap<String, String> = HashMap::new();
    resp.insert(String::from("status"), String::from("ok"));

    Ok(HttpResponse::Ok().json(resp))
}

#[tokio::main]
async fn main() {    
    if let Err(e) = run().await {
        eprintln!("fatal {}", e);
        std::process::exit(1);
    }
}

struct RunContext {
    listen_address: String,
    num_workers: usize,
}

async fn run() -> Result<()> {
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

async fn run_server(env: RunContext) -> Result<(), std::io::Error> {
    println!("run server {}", env.listen_address);
    HttpServer::new(|| {
        App::new()
            .service(healthcheck)
    })
        .bind(env.listen_address)?
        .workers(env.num_workers)
        .run()
        .await
}