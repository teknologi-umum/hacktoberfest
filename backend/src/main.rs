use crate::github::Github;
use actix_web::web::Data;
use actix_web::{App, HttpServer, Result};
use backoff::exponential::ExponentialBackoff;
use backoff::SystemClock;
use chrono::{DateTime, NaiveDate, Utc};
use clap::clap_app;
use config::Config;
use lazy_static::lazy_static;
use scopeguard::defer;
use scraper::run_scrape;
use std::cell::RefCell;
use std::process::exit;
use std::sync::{Arc, Mutex};
use std::{env, io, usize};

mod config;
mod github;
mod handlers;
mod scraper;

use crate::handlers::*;

lazy_static! {
    pub static ref FIRST_OCTOBER: DateTime<Utc> =
        DateTime::<Utc>::from_utc(NaiveDate::from_ymd(2023, 10, 1).and_hms(0, 0, 0), Utc);
    pub static ref LAST_OCTOBER: DateTime<Utc> =
        DateTime::<Utc>::from_utc(NaiveDate::from_ymd(2023, 10, 31).and_hms(23, 59, 59), Utc);
}

#[tokio::main]
async fn main() {
    if cfg!(debug_assertions) {
        color_backtrace::install()
    }
    env_logger::init();

    if let Err(e) = run().await {
        eprintln!("fatal {}", e);
        exit(1);
    }
}

pub struct RunContextInner {}
impl RunContextInner {}

#[derive(Clone)]
pub struct RunContext<'a> {
    pub listen_address: String,
    pub num_workers: usize,
    pub scrape_interval: u64,
    pub github_token: String,

    pub config_path: String,
    pub config: RefCell<Box<Config>>,

    pub scrape_per_page: u8,

    // placeholder
    inner: RefCell<Box<&'a RunContextInner>>,
}

impl<'a> RunContext<'a> {
    pub fn default() -> Self {
        Self {
            inner: RefCell::new(Box::new(&RunContextInner {})),
            listen_address: "127.0.0.1:8080".to_owned(),
            num_workers: 1,
            scrape_interval: 3600,
            github_token: "".to_owned(),
            config_path: "/tmp/data.yml".to_owned(),
            config: RefCell::new(Config::default()),
            scrape_per_page: 100,
        }
    }

    pub fn save_cfg(&self) -> anyhow::Result<Box<Config>> {
        self.config.borrow().clone().save_yaml_to(&self.config_path)
    }
}

pub type RRunContext<'a> = Arc<Mutex<RunContext<'a>>>;

async fn run<'a>() -> Result<()> {
    let app = clap_app!(hacktoberfestd =>
        (version: "")
        (about: "Hacktoberfest serverd")
        (@arg addr: --addr +takes_value "Listen address for HTTP server")
        (@arg wrk: --wrk +takes_value "Number of HTTP server workers")
        (@arg scrape_interval: --("scrape_interval") +takes_value "Scrap interval in second")
        (@arg github_token: --("github_token") +takes_value "Github API Token")
        (@arg config_path: --("config_path") +takes_value "Config path")
        (@arg scrape_per_page: --("scrape_per_page") +takes_value "Github scrap per_page limit")
    )
    .get_matches();

    let default_config = RunContext::default();

    let fallback_laddr = env::var("LISTEN_ADDR").unwrap_or(default_config.listen_address);
    let fallback_num_wrk_str =
        env::var("NUM_WORKERS").unwrap_or(default_config.num_workers.to_string());
    let fallback_num_wrk = fallback_num_wrk_str
        .parse::<usize>()
        .unwrap_or(default_config.num_workers);
    let fallback_scrape_interval_str =
        env::var("scrape_interval").unwrap_or(default_config.scrape_interval.to_string());
    let fallback_scrape_interval = fallback_scrape_interval_str
        .parse::<u64>()
        .unwrap_or(default_config.scrape_interval);
    let fallback_github_token = env::var("GITHUB_TOKEN").unwrap_or(default_config.github_token);
    let fallback_config_path = env::var("CONFIG_PATH").unwrap_or(default_config.config_path);
    let fallback_scrape_per_page_str =
        env::var("scrape_per_page").unwrap_or(default_config.scrape_per_page.to_string());
    let fallback_scrape_per_page = fallback_scrape_per_page_str
        .parse::<u8>()
        .unwrap_or(default_config.scrape_per_page);

    let laddr: String = app.get_one("addr").unwrap_or(&fallback_laddr).to_string();
    let github_token: String = app
        .get_one("github_token")
        .unwrap_or(&fallback_github_token)
        .to_string();
    let num_workers: usize = *app.get_one("wrk").unwrap_or(&fallback_num_wrk);
    let scrape_interval: u64 = *app
        .get_one("scrape_interval")
        .unwrap_or(&fallback_scrape_interval);
    let config_path = app
        .get_one("config_path")
        .unwrap_or(&fallback_config_path)
        .to_string();
    let scrape_per_page: u8 = *app
        .get_one("scrape_per_page")
        .unwrap_or(&fallback_scrape_per_page);

    let conf = RefCell::new(Config::load_or_create(&config_path).unwrap());
    let write_back_conf_path = config_path.clone();

    let env = Arc::new(Mutex::new(RunContext {
        inner: default_config.inner,
        listen_address: laddr.into(),
        num_workers,
        scrape_interval,
        config_path,
        github_token: github_token.clone(),
        config: RefCell::clone(&conf),

        scrape_per_page: scrape_per_page,
    }));

    let defer_ctx = env.clone();
    defer! {
        let local_env = defer_ctx.lock().unwrap();
        println!("write-back config @ {write_back_conf_path}");
        local_env.save_cfg().unwrap();
        println!("> OK");
    }

    let scrape_thread_ctx = env.clone();
    tokio::spawn(async move {
        let github_client = if github_token.is_empty() {
            Github::new()
        } else {
            Github::new_with_token(Some(String::from(github_token)))
        };

        let exponential_backoff_box: Box<ExponentialBackoff<SystemClock>> =
            Box::new(backoff::ExponentialBackoffBuilder::new().build());

        tokio::select! {
            _ret = run_scrape(
                &scrape_thread_ctx,
                exponential_backoff_box,
                &github_client,
            ) => {
                println!("scrap thread stopped unexpectedly");
                return
            }
            _ = tokio::signal::ctrl_c() => {
                println!("scrap thread ended");
                return
            }
        }
    });

    tokio::select! {
        Err(e) = run_server(&env) => Err(e.into()),
        _ = tokio::signal::ctrl_c() => Ok(())
    }
}

async fn run_server<'a>(env: &'a RRunContext<'static>) -> Result<(), io::Error> {
    let data = Data::from(env.clone());
    let local_env = env.lock().unwrap().clone(); // don't hold lock!

    println!("Run server on {}", local_env.listen_address);
    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(healthcheck::handler())
            .service(metrics::handler())
            .service(repositories::handler())
            .service(contributors::handler())
            .service(pullrequest::handler())
    })
    .bind(local_env.listen_address.clone())?
    .workers(local_env.num_workers)
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use std::{thread, time::Duration};

    use scopeguard::defer;

    use crate::RunContext;

    #[test]
    fn test_run_ctxt() {
        let ctx = RunContext::default();
        defer! {
            // borrow, clone, and serialize
            let yaml_repr = ctx.config.borrow().clone().to_string().unwrap();
            println!("CHECK\n{yaml_repr}");
        }
        ctx.config
            .borrow_mut()
            .cached_map
            .insert("todo".to_owned(), "asdaf".to_owned());
    }

    #[tokio::test]
    async fn test_thread_park() -> anyhow::Result<()> {
        tokio::spawn(async move {
            println!("aa");
            thread::park_timeout(Duration::from_secs(5));
            println!("bb");
        })
        .await?;
        Ok(())
    }
}
