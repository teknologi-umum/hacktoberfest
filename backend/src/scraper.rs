use crate::config::ScrapTargetType;
use crate::github::{Github, Issue, Repository};
use crate::github::{GithubError};
use crate::{RunContext};
use chrono::prelude::Local;
use chrono::{DateTime, Utc};
use log::{debug, info, trace};
use scopeguard::defer;
use serde::{Deserialize, Serialize};
use std::cell::{RefCell, Ref};
use std::collections::HashMap;
use std::fmt::Display;
use std::future::Future;
use std::io::Error;
use std::ops::Deref;
use std::process::Output;
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;
use std::time::{Duration, Instant};

#[derive(Serialize, Deserialize)]
pub struct RepositoryCollection {
    pub full_name: String,
    pub html_url: String,
    pub description: Option<String>,
    pub languages: Vec<String>,
    pub stars_count: i64,
    pub forks_count: i64,
    pub topics: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub issues: Vec<Issue>,
}

#[derive(Debug)]
pub enum ScrapError {
    InvalidRepo,
    Github(GithubError),
    Serde(serde_json::Error),
}

pub async fn run_scrape<'a, B>(
    ctx: &'a crate::RRunContext<'_>,
    backoff: B,
    github_client: &Github,
) where
    B: backoff::backoff::Backoff + Clone,
{
    println!("Run scrapper");
    defer! {
        println!("Scrapper loop stopped");
    };
    let scrap_interval;
    {
        let _ctx = ctx.lock().unwrap();
        scrap_interval = _ctx.scrap_interval;
        drop(_ctx);
    }
    loop {
        {
            let _ = backoff::future::retry_notify(
                backoff.clone(),
                || async { Ok(scrape(ctx, github_client).await?) },
                |err, dur| println!("scrape error {:?}: {:?}", dur, err),
            ).await;
            tokio::time::sleep(Duration::from_secs(scrap_interval)).await;
        }
    }
}

pub async fn scrape_inner<'a>(github_client: &Github, username: String, repo: &'a Repository) -> Result<RepositoryCollection, ScrapError> {
    // Skip if there isn't any "hacktoberfest" topic on the repository
    if !repo.topics.contains(&"hacktoberfest".into()) {
        return Err(ScrapError::InvalidRepo);
    }

    log::debug!("Scraping issues for {}", repo.name);
    let issues = github_client
        .list_issues(username.clone(), repo.name.to_owned())
        .await
        .map_err(ScrapError::Github)?
        .into_iter()
        .filter(|issue| issue.labels.iter().any(|l| l.name == "hacktoberfest"))
        .collect::<Vec<Issue>>();

    log::debug!("Scraping languages for {}", repo.name);
    let languages = github_client
        .list_languages(username.clone(), repo.name.to_owned())
        .await
        .map_err(ScrapError::Github)?;

    Ok(RepositoryCollection {
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

pub async fn scrape<'a>(
    ctx: &Arc<Mutex<RunContext<'a>>>,
    github_client: &Github,
) -> Result<(), ScrapError> {
    println!("scrapper start");
    defer! {
        println!("scrapper stop");
    }

    let scrap_targets = {
        ctx.lock().unwrap().config.borrow().scrap_target.clone()
    };
    let scrap_per_page_limit = {
        ctx.lock().unwrap().scrap_per_page
    };
    let mut repository_collection: Vec<RepositoryCollection> = Vec::with_capacity(8);

    for target in scrap_targets
        .into_iter()
        .filter(|t| !t.ignore)
    {
        let username = target.username.clone();

        let mut repository: Vec<Repository> = github_client
            .list_repository(username.clone(), scrap_per_page_limit)
            .await
            .map_err(ScrapError::Github)?;

        // extra filter for Repo target type.
        let repo_target_links = target.target_links();
        if let ScrapTargetType::Repo = target.target_type {
            if repo_target_links.len() < 1 {
                continue;
            }

            repository = repository
                .into_iter()
                .zip(repo_target_links)
                .filter(|(repo, target_link)| {
                    repo.html_url.eq(target_link)
                })
                .map(|(repo, _)| repo)
                .collect();
        }

        for repo in repository.iter() {
            match scrape_inner(github_client, username.clone(), repo).await {
                Ok(coll) => {
                    repository_collection.push(coll);
                },
                Err(e) => {
                    if let ScrapError::InvalidRepo = e {
                        trace!("ignoring {}", repo.full_name);
                        continue;
                    }
                    log::debug!("err {:?} -> {:?}", e, repo);
                }
            }
        }
    }

    

    let json_collection =
        serde_json::to_string::<Vec<RepositoryCollection>>(&repository_collection)
            .map_err(ScrapError::Serde)?;

    {
        let g_ctx = ctx
            .lock()
            .unwrap();
        let mut _cfg = g_ctx.config.borrow_mut();
        _cfg.scrap_last = Some(Local::now());
        _cfg.cached_map
            .insert("repo".to_owned(), json_collection);
    }
    Ok(())
}
