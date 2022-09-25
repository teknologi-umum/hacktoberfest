use std::{sync::Mutex, collections::HashMap};

use actix_web::{web::{Data, self}, Result, HttpResponse, Resource, http::header};
use prometheus::{TextEncoder, Encoder};

async fn metrics(_global_map: Data<Mutex<HashMap<String, String>>>) -> Result<HttpResponse> {
    let enc = TextEncoder::new();
    let mut buf = vec![];
    enc.encode(&prometheus::gather(), &mut buf).expect("failed to encode metrics");

    let response = String::from_utf8(buf.clone()).expect("failed to copy buffer");
    buf.clear();

        Ok(HttpResponse::Ok()
        .insert_header(header::ContentType(mime::TEXT_PLAIN))
        .body(response))
}

pub fn handler() -> Resource {
    web::resource("/metrics").route(web::get().to(metrics))
}

#[cfg(test)]
mod tests {
    use super::metrics;

    #[actix_web::test]
    async fn test_metrics() {

    }
}