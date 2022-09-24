use std::borrow::Borrow;
use std::collections::HashMap;
use std::sync::Mutex;
use actix_web::{get, web::{Data, self}, HttpRequest, Result, HttpResponse, Resource};
use actix_web::http::header::ContentType;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

use crate::github::{DefaultClient, Issue};

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

async fn repositories(mut global_map: Data<Mutex<HashMap<String, String>>>, req: HttpRequest) -> Result<HttpResponse> {
    let unlocked_map = global_map.lock().unwrap();
    let cached = match unlocked_map.get("repo") {
        Some(o) => String::from(o),
        _ => String::new()
    };
    if !String::is_empty(&cached) {
        return Ok(HttpResponse::Ok().content_type(ContentType::json()).body(cached.clone()));
    }

    let mut response: Vec<RepositoriesResponse> = vec![];
    let repository = DefaultClient.list_repository().await.unwrap();
    for repo in repository.iter() {
        // Skip if there isn't any "hacktoberfest" topic on the repository
        if !repo.topics.contains(&String::from("hacktoberfest")) {
            continue
        }

        let issues = DefaultClient.list_issues(repo.name.to_owned()).await.unwrap();
        let languages = DefaultClient.list_languages(repo.name.to_owned()).await.unwrap();

        response.push(RepositoriesResponse{
            full_name: repo.full_name.clone(),
            html_url: repo.html_url.clone(),
            description: repo.description.clone(),
            // TODO: get language list after #3 is merged
            languages,
            stars_count: repo.stargazers_count,
            forks_count: repo.forks_count,
            topics: repo.topics.clone(),
            created_at: repo.created_at,
            updated_at: repo.updated_at,
            issues,
        })
    }

    Ok(HttpResponse::Ok().json(response))
}

pub fn Handler() -> Resource {
    web::resource("/repo")
        .route(web::get().to(repositories))
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use actix_web::{test::TestRequest, http, web::Data};

    use super::repositories;

    #[actix_web::test]
    async fn test_repositories() {
        let local_map = Data::new(HashMap::<String, String>::new());
        let req = TestRequest::default()
            .to_http_request();
        let resp = repositories(local_map, req).await;
        assert_eq!(resp
            .expect("an error occurred")
            .status(), http::StatusCode::OK);
    }
}