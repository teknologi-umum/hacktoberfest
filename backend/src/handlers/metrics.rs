use std::{sync::Mutex, collections::HashMap};

use actix_web::{web::{Data, self}, Result, HttpResponse, Resource, http::header};
use lazy_static::lazy_static;
use prometheus::{
    TextEncoder, Encoder, IntCounterVec, IntGauge, HistogramVec,
    register_int_counter_vec, opts,
    register_int_gauge,
    register_histogram_vec,
};

use crate::RunContext;

const CUSTOM_BUCKETS: &[f64; 14] = &[
    0.0005, 0.0008, 0.00085, 0.0009, 0.00095, 0.001, 0.00105, 0.0011, 0.00115, 0.0012, 0.0015,
    0.002, 0.003, 1.0,
];

lazy_static! {
    pub static ref SCRAPE_COUNT_TOTAL: IntCounterVec = register_int_counter_vec!(
        opts!("scrape_github_count_total", "Scrape count total"),
        &[],
    )
    .expect("Can't create a metric");
    pub static ref SCRAPE_REPO_COUNT_TOTAL: IntCounterVec = register_int_counter_vec!(
        opts!("scrape_github_repo_count_total", "Scrape repository count total"),
        &["repo_username", "repo_name"],
    )
    .expect("Can't create a metric");
    pub static ref SCRAPE_HISTOGRAM_DUR_SECONDS: HistogramVec = register_histogram_vec!(
        "scrape_github_time_seconds", "Scrape duration in seconds",
        &["repo_username", "repo_name"],
        CUSTOM_BUCKETS.to_vec(),
    )
    .expect("Can't create a metric");
}

async fn metrics(_ctx: Data<Mutex<RunContext<'_>>>) -> Result<HttpResponse> {
    let encoder = TextEncoder::new();
    let mut buffer = vec![];
    let fam = &prometheus::gather();
    encoder
        .encode(fam, &mut buffer)
        .expect("Failed to encode metrics");

    let response = String::from_utf8(buffer.clone()).expect("Failed to convert bytes to string");
    buffer.clear();

    Ok(HttpResponse::Ok()
        .insert_header(header::ContentType(mime::TEXT_PLAIN))
        .body(response))
}

pub fn handler() -> Resource {
    web::resource("/metrics").route(web::get().to(metrics))
}

#[cfg(test)]
mod tests {
    use crate::handlers::{SCRAPE_COUNT_TOTAL, SCRAPE_REPO_COUNT_TOTAL};

    #[actix_web::test]
    async fn test_metrics() {
        SCRAPE_COUNT_TOTAL.with_label_values(&[]).inc();
        SCRAPE_REPO_COUNT_TOTAL.with_label_values(&["username", "repo"]).inc();
        println!("{:?}", prometheus::default_registry().gather());
    }
}