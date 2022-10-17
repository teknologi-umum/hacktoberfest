use actix_web::{
    http,
    web::{self, Data},
    HttpRequest, HttpResponse, Resource, Result,
};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

use crate::RunContext;

#[derive(Serialize, Deserialize)]
pub struct ContributorResponse {
    pub full_name: String,
    pub profile_url: String,
    pub merged_pulls: i64,
    pub pending_pulls: i64,
}

async fn contributors(ctx: Data<Mutex<RunContext<'_>>>, _req: HttpRequest) -> Result<HttpResponse> {
    let unlocked_ctx = ctx.lock().unwrap();
    let unlocked_map = &unlocked_ctx.config.borrow().cached_map;
    let cached: String = match unlocked_map.get("contributors") {
        Some(cached_repo) => cached_repo.into(),
        _ => "[]".into(),
    };

    Ok(HttpResponse::Ok()
        .content_type(http::header::ContentType::json())
        .body(cached))
}

pub fn handler() -> Resource {
    web::resource("/contrib").route(web::get().to(contributors))
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use actix_web::{http, test::TestRequest, web::Data};

    use crate::RunContext;

    use super::contributors;

    #[actix_web::test]
    async fn test_contributors() {
        let ctx = Data::new(Mutex::new(RunContext::default()));
        let req = TestRequest::default().to_http_request();
        let resp = contributors(ctx, req).await;
        assert_eq!(
            resp.expect("an error occurred").status(),
            http::StatusCode::OK
        );
    }
}
