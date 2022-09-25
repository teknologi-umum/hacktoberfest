use actix_web::{
    http,
    web::{self, Data},
    HttpRequest, HttpResponse, Resource, Result,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;

use crate::github::Issue;

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

async fn repositories(
    global_map: Data<Mutex<HashMap<String, String>>>,
    _req: HttpRequest,
) -> Result<HttpResponse> {
    let unlocked_map = global_map.lock().unwrap();
    let cached: String = match unlocked_map.get("repo") {
        Some(cached_repo) => cached_repo.into(),
        _ => "[]".into(),
    };

    Ok(HttpResponse::Ok()
        .content_type(http::header::ContentType::json())
        .body(cached))
}

pub fn handler() -> Resource {
    web::resource("/repo").route(web::get().to(repositories))
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::sync::Mutex;

    use actix_web::{http, test::TestRequest, web::Data};

    use super::repositories;

    #[actix_web::test]
    async fn test_repositories() {
        let local_map = Data::new(Mutex::new(HashMap::<String, String>::new()));
        let req = TestRequest::default().to_http_request();
        let resp = repositories(local_map, req).await;
        assert_eq!(
            resp.expect("an error occurred").status(),
            http::StatusCode::OK
        );
    }
}
