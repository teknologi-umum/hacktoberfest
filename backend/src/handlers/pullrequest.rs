use crate::RunContext;
use actix_web::web::Data;
use actix_web::{http, web, HttpRequest, HttpResponse, Resource, Result};
use std::sync::Mutex;

async fn pullrequest(ctx: Data<Mutex<RunContext<'_>>>, _req: HttpRequest) -> Result<HttpResponse> {
    let unlocked_ctx = ctx.lock().unwrap();
    let unlocked_map = &unlocked_ctx.config.borrow().cached_map;
    let cached: String = match unlocked_map.get("pull_request") {
        Some(cached_pr) => cached_pr.into(),
        None => "[]".into(),
    };

    Ok(HttpResponse::Ok()
        .content_type(http::header::ContentType::json())
        .body(cached))
}

pub fn handler() -> Resource {
    web::resource("/pullrequest").route(web::get().to(pullrequest))
}

#[cfg(test)]
mod tests {
    use crate::pullrequest::pullrequest;
    use crate::RunContext;
    use actix_web::http;
    use actix_web::test::TestRequest;
    use actix_web::web::Data;
    use std::sync::Mutex;

    #[actix_web::test]
    async fn test_pullrequest() {
        let ctx = Data::new(Mutex::new(RunContext::default()));
        let req = TestRequest::default().to_http_request();
        let resp = pullrequest(ctx, req).await;
        assert_eq!(
            resp.expect("an error occured").status(),
            http::StatusCode::OK
        );
    }
}
