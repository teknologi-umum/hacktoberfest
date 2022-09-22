use actix_web::{App, get, HttpResponse, HttpServer, web, Result};
use std::{io, env};
use std::collections::HashMap;
use std::process::exit;

mod github;

#[get("/health")]
async fn healthcheck() -> Result<HttpResponse> {
    let mut resp: HashMap<String, String> = HashMap::new();
    resp.insert(String::from("status"), String::from("ok"));

    Ok(HttpResponse::Ok().json(resp))
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    let listen_address = env::var("LISTEN_ADDRESS").unwrap_or(String::from("127.0.0.1:8080"));
    let num_workers_str = env::var("NUM_WORKERS").unwrap_or(String::from("1"));
    let mut num_workers: usize;

    match num_workers_str.parse::<usize>() {
                Err(e) => {
                    println!("NUM_WORKERS is not a number!")
                    exit(1);
                }
        Ok(n) => {
            num_workers = n;
        }

    }
    HttpServer::new(|| {
        App::new()
            .service(healthcheck)
    })
        .bind(listen_address)?
        .workers(num_workers)
        .run()
        .await
}
