use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::DefaultClient;
use crate::github::Issue;

#[derive(Serialize, Deserialize)]
pub struct RepositoryCollection {
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

pub async fn scrape(global_map: &Arc<Mutex<HashMap<String, String>>>) {
    println!("Scraping...");
    let mut repository_collection: Vec<RepositoryCollection> = vec![];
    let repository = DefaultClient.list_repository().await.unwrap();
    for repo in repository.iter() {
        // Skip if there isn't any "hacktoberfest" topic on the repository
        if !repo.topics.contains(&"hacktoberfest".into()) {

            continue
        }

        let issues = DefaultClient.list_issues(repo.name.to_owned()).await.unwrap();
        let languages = DefaultClient.list_languages(repo.name.to_owned()).await.unwrap();

        repository_collection.push(RepositoryCollection {
            full_name: repo.full_name.clone(),
            html_url: repo.html_url.clone(),
            description: repo.description.clone(),
            languages,
            stars_count: repo.stargazers_count,
            forks_count: repo.forks_count,
            topics: repo.topics.clone(),
            created_at: repo.created_at,
            updated_at: repo.updated_at,
            issues,
        })
    }

    let json_collection = serde_json::to_string::<Vec<RepositoryCollection>>(&repository_collection).unwrap();

    global_map.lock().unwrap().insert(String::from("repo"), json_collection);
    println!("Scraped!");
}