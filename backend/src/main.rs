use crate::github::Github;
use actix_web::web::Data;
use actix_web::{App, HttpServer, Result};
use backoff::exponential::ExponentialBackoff;
use backoff::SystemClock;
use clap::clap_app;
use scraper::run_scrape;
use std::collections::HashMap;
use std::process::exit;
use std::sync::{Arc, Mutex};
use std::{env, io, usize};

mod github;
mod handlers;
mod scraper;

use crate::handlers::*;

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("fatal {}", e);
        exit(1);
    }
}

#[derive(Clone)]
pub struct RunContext {
    listen_address: String,
    num_workers: usize,
    scrap_interval: u64,
}

async fn run() -> Result<()> {
    if cfg!(debug_assertions) {
        color_backtrace::install()
    }

    handlers::init();

    let app = clap_app!(hacktoberfestd =>
        (version: "")
        (about: "Hacktoberfest serverd")
        (@arg addr: --addr +takes_value "Listen address for HTTP server")
        (@arg wrk: --wrk +takes_value "Number of HTTP server workers")
        (@arg scrap_interval: --("scrap-interval") +takes_value "Scrap interval in second")
        (@arg github_token: --("github_token") +takes_value "Github API Token")
    )
    .get_matches();

    let falback_laddr = env::var("LISTEN_ADDR").unwrap_or("127.0.0.1:8080".into());
    let fallback_num_wrk_str = env::var("NUM_WORKERS").unwrap_or("1".into());
    let fallback_num_wrk = fallback_num_wrk_str.parse::<usize>().unwrap_or(1);
    let fallback_scrap_interval_str = env::var("SCRAP_INTERVAL").unwrap_or("3600".into());
    let fallback_scrap_interval = fallback_scrap_interval_str.parse::<u64>().unwrap_or(3600);
    let fallback_github_token = env::var("GITHUB_TOKEN").unwrap_or("".into());

    let laddr: String = app.get_one("addr").unwrap_or(&falback_laddr).to_string();
    let github_token: String = app
        .get_one("github_token")
        .unwrap_or(&fallback_github_token)
        .to_string();
    let num_wrk: usize = *app.get_one("wrk").unwrap_or(&fallback_num_wrk);
    let scrap_interval: u64 = *app
        .get_one("scrap_interval")
        .unwrap_or(&fallback_scrap_interval);

    let env = RunContext {
        listen_address: laddr.into(),
        num_workers: num_wrk,
        scrap_interval,
    };

    let global_map = Arc::new(Mutex::new(HashMap::<String, String>::new()));
    let server_map = Arc::clone(&global_map);
    let scraper_map = Arc::clone(&global_map);

    let run_context_clone: RunContext = env.clone();
    tokio::spawn(async move {
        let github_client = if github_token.is_empty() {
            Github::new()
        } else {
            Github::new_with_token(Some(String::from(github_token)))
        };

        let exponential_backoff_box: Box<ExponentialBackoff<SystemClock>> =
            Box::new(backoff::ExponentialBackoffBuilder::new().build());

        run_scrape(
            run_context_clone,
            exponential_backoff_box,
            &scraper_map,
            &github_client,
        )
        .await;
    });

    if let Err(e) = run_server(env, Data::from(server_map)).await {
        return Err(e.into());
    }

    Ok(())
}

async fn run_server(
    env: RunContext,
    global_map: Data<Mutex<HashMap<String, String>>>,
) -> Result<(), io::Error> {
    println!("Run server on: {}", env.listen_address);

    HttpServer::new(move || {
        App::new()
            .app_data(global_map.clone())
            .service(healthcheck::handler())
            .service(metrics::handler())
            .service(repositories::handler())
    })
    .bind(env.listen_address)?
    .workers(env.num_workers)
    .run()
    .await
}
