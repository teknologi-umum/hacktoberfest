use actix_web::web::Data;
use actix_web::{web, HttpResponse, Resource, Result};
use std::collections::HashMap;
use std::sync::Mutex;

use crate::RunContext;

async fn healthcheck(_ctx: Data<Mutex<RunContext<'_>>>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(HashMap::from([("status", "ok")])))
}

pub fn handler() -> Resource {
    web::resource("/healthz").route(web::get().to(healthcheck))
}

#[cfg(test)]
mod tests {
    use actix_web::http;
    use actix_web::web::Data;
    use std::sync::Mutex;

    use crate::RunContext;

    use super::healthcheck;

    #[actix_web::test]
    async fn test_healthcheck() {
        let ctx = Data::new(Mutex::new(RunContext::default()));
        let resp = healthcheck(ctx).await;
        assert_eq!(
            resp.expect("an error occurred").status(),
            http::StatusCode::OK
        );
    }
}
