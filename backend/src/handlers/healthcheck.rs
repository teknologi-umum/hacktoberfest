use actix_web::{
    http,
    Result,
    HttpRequest, HttpResponse, web, Resource,
};
use actix_web::web::Data;
use std::collections::HashMap;
use std::sync::Mutex;

async fn healthcheck(global_map: Data<Mutex<HashMap<String, String>>>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(HashMap::from([("status", "ok")])))
}

pub fn Handler() -> Resource {
    web::resource("/healthz")
        .route(web::get().to(healthcheck))
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
        let resp = healthcheck(local_map).await;
        assert_eq!(resp
                .expect("an error occurred")
                .status(), http::StatusCode::OK);
    }
}