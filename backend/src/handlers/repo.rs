use std::collections::HashMap;
use actix_web::{get, web::Data, HttpRequest, Result, HttpResponse};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

use crate::github::{Github, Issue};

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

#[get("/repo")]
pub async fn repositories(global_map: Data<HashMap<String, String>>, req: HttpRequest) -> Result<HttpResponse> {
    let gh = Github::new();

    let mut response: Vec<RepositoriesResponse> = vec![];

    let repository = gh.list_repository().await.unwrap();

    for repo in repository.iter() {
        let issues = gh.list_issues(repo.name.clone()).await.unwrap();
        response.push(RepositoriesResponse{
            full_name: repo.full_name.clone(),
            html_url: repo.html_url.clone(),
            description: repo.description.clone(),
            // TODO: get language list after #3 is merged
            languages: vec![],
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
