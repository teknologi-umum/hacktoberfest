use crate::github::{Issue, GithubError};
use crate::{DEFAULT_CLIENT, RunContext};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Display;
use std::future::Future;
use std::io::Error;
use std::process::Output;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

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

pub enum ScrapError {
    Github(GithubError),
    Serde(serde_json::Error),
}
impl Display for ScrapError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

pub async fn run_scrape<B> (ctx: RunContext, bf: B, global_map: &Arc<Mutex<HashMap<String, String>>>)
where
    B: backoff::backoff::Backoff + Clone,
{
    println!("run scrapper");
    loop {
        backoff::future::retry_notify(bf.clone(), || async {
            Ok(scrape(global_map).await?)
        }, |err, dur| println!("scrape error {:?}: {}", dur, err));
        thread::sleep(Duration::new(ctx.scrap_interval, 0));
    }
}

pub async fn scrape(global_map: &Arc<Mutex<HashMap<String, String>>>) -> Result<(), ScrapError> {
    let mut repository_collection: Vec<RepositoryCollection> = vec![];
    let repository = DEFAULT_CLIENT.list_repository().await.map_err(ScrapError::Github)?;
    for repo in repository.iter() {
        // Skip if there isn't any "hacktoberfest" topic on the repository
        if !repo.topics.contains(&"hacktoberfest".into()) {
            continue;
        }

        let issues = DEFAULT_CLIENT
            .list_issues(repo.name.to_owned())
            .await
            .map_err(ScrapError::Github)?;
        let languages = DEFAULT_CLIENT
            .list_languages(repo.name.to_owned())
            .await
            .map_err(ScrapError::Github)?;

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

    let json_collection =
        serde_json::to_string::<Vec<RepositoryCollection>>(&repository_collection).map_err(ScrapError::Serde)?;

    global_map
        .lock()
        .unwrap()
        .insert("repo".into(), json_collection);

    Ok(())
}
