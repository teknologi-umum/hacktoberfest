use actix_web::{
    get,
    http,
    Result,
    HttpRequest, HttpResponse,
};
use actix_web::web::Data;
use std::collections::HashMap;

#[get("/healthz")]
async fn healthcheck(global_map: Data<HashMap<String, String>>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(HashMap::from([("status", "ok")])))
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use actix_web::{http, test};
    use actix_web::web::{Data};

    use super::healthcheck;

    #[actix_web::test]
    async fn test_healthcheck() {
        let local_map = Data::new(HashMap::<String, String>::new());
        // HMM?! kena macro rule `get`
        // let a = healthcheck{};
        // let resp = healthcheck(local_map).await;
        // assert_eq!(resp.status(), http::StatusCode::OK);
    }
}