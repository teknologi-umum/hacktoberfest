use crate::config::ScrapTargetType;
use crate::github::{Github, Issue, Repository};
use crate::github::{GithubError, PullRequest};
use crate::RunContext;
use chrono::prelude::Local;
use chrono::{DateTime, NaiveDate, Utc};
use log::trace;
use scopeguard::defer;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

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

#[derive(Serialize, Deserialize)]
pub struct ContributorCollection {
    pub full_name: String,
    pub profile_url: String,
    pub merged_pulls: i64,
    pub pending_pulls: i64,
}

#[derive(Debug)]
pub enum ScrapError {
    InvalidRepo,
    Github(GithubError),
    Serde(serde_json::Error),
}

pub async fn run_scrape<'a, B>(ctx: &'a crate::RRunContext<'_>, backoff: B, github_client: &Github)
where
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
            )
            .await;
            tokio::time::sleep(Duration::from_secs(scrap_interval)).await;
        }
    }
}

pub async fn scrape_repository_collection<'a>(
    github_client: &Github,
    username: String,
    repo: &'a Repository,
) -> Result<RepositoryCollection, ScrapError> {
    // Skip if there isn't any "hacktoberfest" topic on the repository
    if !repo.topics.contains(&"hacktoberfest".into()) {
        return Err(ScrapError::InvalidRepo);
    }

    log::debug!("Scraping issues for {}", repo.name);
    let issues: Vec<Issue> = github_client
        .list_issues(username.clone(), repo.name.to_owned())
        .await
        .map_err(ScrapError::Github)?
        .into_iter()
        .filter(|issue| issue.labels.iter().any(|l| l.name == "hacktoberfest"))
        .collect::<Vec<Issue>>();

    log::debug!("Scraping languages for {}", repo.name);
    let languages: Vec<String> = github_client
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

pub async fn scrape_contributor_collection(
    github_client: &Github,
    username: String,
    repo: &Repository,
) -> Result<Vec<ContributorCollection>, ScrapError> {
    // Skip if there isn't any "hacktoberfest" topic on the repository
    if !repo.topics.contains(&"hacktoberfest".into()) {
        return Err(ScrapError::InvalidRepo);
    }

    log::debug!("Scraping PRs for {}", repo.name);

    let pulls: Vec<PullRequest> = github_client
        .list_pull_request(username.clone(), repo.name.to_owned())
        .await
        .map_err(ScrapError::Github)?;

    let mut contributor_map = HashMap::<String, ContributorCollection>::new();

    let first_october =
        DateTime::<Utc>::from_utc(NaiveDate::from_ymd(2022, 10, 1).and_hms(0, 0, 0), Utc);
    let last_october =
        DateTime::<Utc>::from_utc(NaiveDate::from_ymd(2022, 10, 31).and_hms(23, 59, 59), Utc);

    for pull in pulls.iter() {
        // Skip if pull request was created before 1 Oct 2022
        if pull.created_at.lt(&first_october) || pull.created_at.gt(&last_october) {
            continue;
        }

        let merged: bool = matches!(
            pull.merged_at,
            Some(date) if date.gt(&first_october) && date.lt(&last_october));

        match contributor_map.get_mut(&pull.user.login.clone()) {
            Some(contributor) => {
                contributor.merged_pulls = if merged {
                    contributor.merged_pulls + 1
                } else {
                    contributor.merged_pulls
                };

                contributor.pending_pulls = if !merged {
                    contributor.pending_pulls + 1
                } else {
                    contributor.pending_pulls
                };
            }
            _ => {
                let pending_pulls: i64 = if merged { 0 } else { 1 };
                let merged_pulls: i64 = if merged { 1 } else { 0 };
                contributor_map.insert(
                    pull.user.login.clone(),
                    ContributorCollection {
                        full_name: pull.user.login.clone(),
                        profile_url: pull.user.html_url.clone(),
                        pending_pulls,
                        merged_pulls,
                    },
                );
            }
        }
    }

    let contributors: Vec<ContributorCollection> =
        contributor_map.into_iter().map(|(_, c)| c).collect();

    Ok(contributors)
}

pub async fn scrape<'a>(
    ctx: &Arc<Mutex<RunContext<'a>>>,
    github_client: &Github,
) -> Result<(), ScrapError> {
    println!("scrapper start");
    defer! {
        println!("scrapper stop");
    }

    let scrap_targets = { ctx.lock().unwrap().config.borrow().scrap_target.clone() };
    let scrap_per_page_limit = { ctx.lock().unwrap().scrap_per_page };
    let mut repository_collection: Vec<RepositoryCollection> = Vec::with_capacity(8);
    let mut contributor_map: HashMap<String, ContributorCollection> =
        HashMap::<String, ContributorCollection>::new();

    for target in scrap_targets.into_iter().filter(|t| !t.ignore) {
        let username = target.username.clone();

        let mut repository: Vec<Repository> = github_client
            .list_repository(username.clone(), scrap_per_page_limit)
            .await
            .map_err(ScrapError::Github)?;

        // extra filter for Repo target type.
        let repo_target_links: Vec<String> = target.target_links();
        if let ScrapTargetType::Repo = target.target_type {
            if repo_target_links.len() < 1 {
                continue;
            }

            repository = repository
                .into_iter()
                .zip(repo_target_links)
                .filter(|(repo, target_link)| repo.html_url.eq(target_link))
                .map(|(repo, _)| repo)
                .collect();
        }

        for repo in repository.iter() {
            match scrape_repository_collection(github_client, username.clone(), repo).await {
                Ok(coll) => {
                    repository_collection.push(coll);
                }
                Err(e) => {
                    if let ScrapError::InvalidRepo = e {
                        trace!("ignoring {}", repo.full_name);
                        continue;
                    }
                    log::debug!("err {:?} -> {:?}", e, repo);
                }
            }

            match scrape_contributor_collection(github_client, username.clone(), repo).await {
                Ok(collections) => {
                    for contributor in collections.iter() {
                        match contributor_map.get_mut(&contributor.full_name) {
                            Some(c) => {
                                c.merged_pulls += contributor.merged_pulls;
                                c.pending_pulls += contributor.pending_pulls;
                            }
                            _ => {
                                contributor_map.insert(
                                    contributor.full_name.clone(),
                                    ContributorCollection {
                                        full_name: contributor.full_name.clone(),
                                        profile_url: contributor.profile_url.clone(),
                                        merged_pulls: contributor.merged_pulls,
                                        pending_pulls: contributor.pending_pulls,
                                    },
                                );
                            }
                        }
                    }
                }
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

    let contributor_collection: Vec<ContributorCollection> =
        contributor_map.into_iter().map(|(_, c)| c).collect();

    let repository_json_collection: String =
        serde_json::to_string::<Vec<RepositoryCollection>>(&repository_collection)
            .map_err(ScrapError::Serde)?;
    let contributor_json_collection: String =
        serde_json::to_string::<Vec<ContributorCollection>>(&contributor_collection)
            .map_err(ScrapError::Serde)?;

    {
        let g_ctx = ctx.lock().unwrap();
        let mut _cfg = g_ctx.config.borrow_mut();
        _cfg.scrap_last = Some(Local::now());
        _cfg.cached_map
            .insert("repo".to_owned(), repository_json_collection);
        _cfg.cached_map
            .insert("contributors".to_owned(), contributor_json_collection);
    }
    Ok(())
}
