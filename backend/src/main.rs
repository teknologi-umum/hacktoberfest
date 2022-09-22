use clap::{clap_app, value_t};
use actix_web::{App, get, HttpResponse, HttpServer, web, Result, HttpRequest};
use core::num;
use serde::{Deserialize, Serialize};
use std::{io, env, usize};
use std::collections::HashMap;
use std::process::exit;
use actix_web::web::Data;
use chrono::{DateTime, Utc};
use crate::github::{Github, Issue};

mod github;

#[get("/health")]
async fn healthcheck(global_map: Data<HashMap<String, String>>) -> Result<HttpResponse> {
    let mut resp: HashMap<String, String> = HashMap::new();
    resp.insert(String::from("status"), String::from("ok"));

    Ok(HttpResponse::Ok().json(resp))
}

#[derive(Serialize, Deserialize)]
pub struct RepositoriesResponse {
    pub full_name: String,
    pub html_url: String,
    pub description: String,
    pub languages: Vec<String>,
    pub stars_count: i64,
    pub forks_count: i64,
    pub topics: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub issues: Vec<Issue>,
}

#[get("/repo")]
async fn repositories(global_map: Data<HashMap<String, String>>, req: HttpRequest) -> Result<HttpResponse> {
    let gh = Github::new();

    let mut response: Vec<RepositoriesResponse> = vec![];

    let repository = gh.list_repository().await.unwrap();

    for repo in repository.iter() {
        let issues = gh.list_issues(repo.name.clone()).await.unwrap();
        response.push(RepositoriesResponse{
            full_name: repo.full_name.clone(),
            html_url: repo.html_url.clone(),
            description: repo.description.clone(),
            // TODO: get language list after #3 is merged
            languages: vec![],
            stars_count: repo.stargazers_count,
            forks_count: repo.forks_count,
            topics: repo.topics.clone(),
            created_at: repo.created_at,
            updated_at: repo.updated_at,
            issues,
        })
    }

    Ok(HttpResponse::Ok().json(response))
}

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
    let global_map = Data::new(HashMap::<String, String>::new());
    HttpServer::new(|| {
        App::new()
            .app_data(global_map.clone())
            .service(healthcheck)
    })
        .bind(env.listen_address)?
        .workers(env.num_workers)
        .run()
        .await
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use actix_web::http::header::ContentType;
    use actix_web::{http, test};
    use actix_web::web::Data;
    use crate::healthcheck;


    #[actix_web::test]
    async fn test_healthcheck() {
        let local_map = Data::new(HashMap::<String, String>::new());
        let resp = healthcheck(local_map).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }
}