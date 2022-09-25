use actix_web::web::Data;
use actix_web::{web, HttpResponse, Resource, Result};
use std::collections::HashMap;
use std::sync::Mutex;

async fn healthcheck(_global_map: Data<Mutex<HashMap<String, String>>>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(HashMap::from([("status", "ok")])))
}

pub fn handler() -> Resource {
    web::resource("/healthz").route(web::get().to(healthcheck))
}

#[cfg(test)]
mod tests {
    use actix_web::http;
    use actix_web::web::Data;
    use std::collections::HashMap;
    use std::sync::Mutex;

    use super::healthcheck;

    #[actix_web::test]
    async fn test_healthcheck() {
        let local_map = Data::new(Mutex::new(HashMap::<String, String>::new()));
        let resp = healthcheck(local_map).await;
        assert_eq!(
            resp.expect("an error occurred").status(),
            http::StatusCode::OK
        );
    }
}
