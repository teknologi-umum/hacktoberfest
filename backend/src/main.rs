use actix_web::web::Data;
use clap::{clap_app, value_t};
use actix_web::{App, HttpServer, Result};
use std::{io, env, usize, thread};
use std::process::exit;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use crate::github::{DefaultClient, Github};

mod github;
mod handlers;
mod scraper;

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
    scrap_interval: u64,
}

async fn run() -> Result<()> {
    if cfg!(debug_assertions) { color_backtrace::install() }

    let app = clap_app!(hacktoberfestd =>
        (version: "")
        (about: "Hacktoberfest serverd")
        (@arg addr: --addr +takes_value "Listen address for HTTP server")
        (@arg wrk: --wrk +takes_value "Number of HTTP server workers")
        (@arg scrap_interval: --("scrap-intarval") +takes_value "Scrap interval in second")
        (@arg github_token: --("github_token") +takes_value "Github API Token")
    ).get_matches();
    
    let falback_laddr = env::var("LISTEN_ADDR").unwrap_or("127.0.0.1:8080".into());
    let fallback_num_wrk_str = env::var("NUM_WORKERS").unwrap_or("1".into());
    let fallback_scrap_interval_str = env::var("SCRAP_INTERVAL").unwrap_or("3600".into());
    let fallback_github_token = env::var("GITHUB_TOKEN").unwrap_or("".into());
    let mut fallback_num_wrk = 1;
    if let Ok(num_wrk) = fallback_num_wrk_str.parse::<usize>() {
        fallback_num_wrk = num_wrk;
    }
    let mut fallback_scrap_interval = 3600;
    if let Ok(num_scrap_inv) = fallback_scrap_interval_str.parse::<u64>() {
        fallback_scrap_interval = num_scrap_inv;
    }

    let laddr = app.value_of("addr").unwrap_or(&falback_laddr[..]);
    let github_token = app.value_of("github_token").unwrap_or(&fallback_github_token[..]);
    let num_wrk = value_t!(app, "wrk", usize).unwrap_or(fallback_num_wrk);
    let scrap_interval = value_t!(app, "scrap_interval", u64).unwrap_or(fallback_scrap_interval);

    let env = RunContext {
        listen_address: laddr.into(),
        num_workers: num_wrk,
        scrap_interval,
    };


    let global_map = Arc::new(Mutex::new(HashMap::<String, String>::new()));
    let server_map = Arc::clone(&global_map);
    let scraper_map = Arc::clone(&global_map);

    tokio::spawn( async move {
        let github_client = if github_token.is_empty() { Github::new() }
            else { Github::new_with_token(Some(String::from(github_token))) };

        println!("run scrapper");
        loop {
            scraper::scrape(&scraper_map, &github_client).await;
            thread::sleep(Duration::new(scrap_interval, 0));
        }
    });

    if let Err(e) = run_server(env, Data::from(server_map)).await {
        return Err(e.into())
    }

    Ok(())
}

async fn run_server(env: RunContext, global_map: Data<Mutex<HashMap<String, String>>>) -> Result<(), io::Error> {
    println!("run server {}", env.listen_address);

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
