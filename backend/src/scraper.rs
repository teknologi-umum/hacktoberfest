use crate::config::ScrapeTargetType;
use crate::github::{Github, Issue, Repository, User};
use crate::github::{GithubError, PullRequest};
use crate::{RunContext, FIRST_OCTOBER, LAST_OCTOBER};
use chrono::prelude::Local;
use chrono::{DateTime, Utc};
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

#[derive(Serialize, Deserialize)]
pub enum PullRequestState {
    Open,
    Closed,
}

#[derive(Serialize, Deserialize)]
pub enum PullRequestMergeableState {
    Unknown,
    Dirty,
    Clean,
}

#[derive(Serialize, Deserialize)]
pub enum PullRequestAuthorAssociation {
    FirstTimeContributor,
    Contributor,
    Member,
    Owner,
    Unknown,
}

#[derive(Serialize, Deserialize)]
pub struct PullRequestDiff {
    pub additions: i64,
    pub deletions: i64,
    pub changed_files: i64,
}

#[derive(Serialize, Deserialize)]
pub struct PullRequestCollection {
    pub number: i64,
    pub html_url: String,
    pub title: String,
    pub state: PullRequestState,
    pub mergeable_state: PullRequestMergeableState,
    pub locked: bool,
    pub user: User,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub merged_at: DateTime<Utc>,
    pub closed_at: DateTime<Utc>,
    pub merged: bool,
    pub draft: bool,
    pub requested_reviewers: Vec<User>,
    pub author_association: PullRequestAuthorAssociation,
    pub comments: i64,
    pub review_comments: i64,
    pub diff: PullRequestDiff,
}

#[derive(Debug)]
pub enum ScrapeError {
    InvalidRepo,
    Github(GithubError),
    Serde(serde_json::Error),
}

pub async fn run_scrape<'a, B>(ctx: &'a crate::RRunContext<'_>, backoff: B, github_client: &Github)
where
    B: backoff::backoff::Backoff + Clone,
{
    println!("Run scraper");
    defer! {
        println!("scraper loop stopped");
    };

    let scrape_interval;
    {
        let _ctx = ctx.lock().unwrap();
        scrape_interval = _ctx.scrape_interval;
        drop(_ctx);
    }

    loop {
        let _ = backoff::future::retry_notify(
            backoff.clone(),
            || async { Ok(scrape(ctx, github_client).await?) },
            |err, dur| println!("scrape error {:?}: {:?}", dur, err),
        )
        .await;
        tokio::time::sleep(Duration::from_secs(scrape_interval)).await;
    }
}

pub async fn scrape_repository_collection(
    github_client: &Github,
    username: &String,
    repo: &Repository,
) -> Result<RepositoryCollection, ScrapeError> {
    // Skip if there isn't any "hacktoberfest" topic on the repository
    if !repo.topics.contains(&"hacktoberfest".into()) {
        return Err(ScrapeError::InvalidRepo);
    }

    log::debug!("Scraping issues for {}", repo.name);
    let issues: Vec<Issue> = github_client
        .list_issues(&username, &repo.name)
        .await
        .map_err(ScrapeError::Github)?
        .into_iter()
        .filter(|issue| issue.labels.iter().any(|l| l.name == "hacktoberfest"))
        .collect::<Vec<Issue>>();

    log::debug!("Scraping languages for {}", repo.name);
    let languages: Vec<String> = github_client
        .list_languages(&username, &repo.name)
        .await
        .map_err(ScrapeError::Github)?;

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
    pulls: &[PullRequest],
) -> Result<Vec<ContributorCollection>, ScrapeError> {
    let contributors: Vec<ContributorCollection> = pulls
        .iter()
        .filter(|pull| pull.created_at.ge(&FIRST_OCTOBER) && pull.created_at.le(&LAST_OCTOBER))
        .fold(HashMap::<String, ContributorCollection>::new(), |mut contributors_map, pull| {
            let merged: bool = matches!(pull.merged_at, Some(date) if date.gt(&FIRST_OCTOBER) && date.lt(&LAST_OCTOBER));            
            match contributors_map.get_mut(&pull.user.login) {
                Some(contributor) => {
                    if merged { contributor.merged_pulls += 1 }
                    if !merged { contributor.pending_pulls += 1 }
                }
                _ => {
                    contributors_map.insert(
                        pull.user.login.clone(),
                        ContributorCollection {
                            full_name: pull.user.login.clone(),
                            profile_url: pull.user.html_url.clone(),
                            pending_pulls: merged.into(),
                            merged_pulls: (!merged).into(),
                        },
                    );
                }
            };
            contributors_map
        })
        .into_values()
        .collect();

    Ok(contributors)
}

pub async fn scrape_pull_request(
    github_client: &Github,
    username: &String,
    repo: &Repository,
    number: i64,
) -> Result<PullRequestCollection, ScrapeError> {
    let pr: PullRequest = github_client
        .pull_request(username, &repo.name, number)
        .await
        .map_err(ScrapeError::Github)?;

    let pull_request = PullRequestCollection {
        number: pr.number,
        html_url: pr.html_url,
        title: pr.title,
        state: match pr.state.as_str() {
            "open" => PullRequestState::Open,
            _ => PullRequestState::Closed,
        },
        mergeable_state: match pr.mergeable_state {
            Some(state) => match state.as_str() {
                "clean" => PullRequestMergeableState::Clean,
                "dirty" => PullRequestMergeableState::Dirty,
                _ => PullRequestMergeableState::Unknown,
            },
            None => PullRequestMergeableState::Unknown,
        },
        locked: pr.locked,
        user: pr.user,
        created_at: pr.created_at,
        updated_at: pr.updated_at,
        merged_at: pr.merged_at.unwrap_or(DateTime::<Utc>::MIN_UTC),
        closed_at: pr.closed_at.unwrap_or(DateTime::<Utc>::MIN_UTC),
        merged: pr.merged.unwrap_or(false),
        draft: pr.draft.unwrap_or(false),
        requested_reviewers: match pr.requested_reviewers {
            Some(requested_reviewers) => requested_reviewers,
            None => Vec::<User>::new(),
        },
        author_association: match pr.author_association {
            Some(author_association) => match author_association.as_str() {
                "FIRST_TIME_CONTRIBUTOR" => PullRequestAuthorAssociation::FirstTimeContributor,
                "CONTRIBUTOR" => PullRequestAuthorAssociation::Contributor,
                "MEMBER" => PullRequestAuthorAssociation::Member,
                "OWNER" => PullRequestAuthorAssociation::Owner,
                _ => PullRequestAuthorAssociation::Unknown,
            },
            None => PullRequestAuthorAssociation::Unknown,
        },
        comments: pr.comments.unwrap_or(0),
        review_comments: pr.review_comments.unwrap_or(0),
        diff: PullRequestDiff {
            additions: pr.additions.unwrap_or(0),
            deletions: pr.deletions.unwrap_or(0),
            changed_files: pr.changed_files.unwrap_or(0),
        },
    };

    Ok(pull_request)
}

pub async fn scrape<'a>(
    ctx: &Arc<Mutex<RunContext<'a>>>,
    github_client: &Github,
) -> Result<(), ScrapeError> {
    println!("scraper start");
    defer! {
        println!("scraper stop");
    }

    let scrape_targets = { ctx.lock().unwrap().config.borrow().scrape_target.clone() };
    let scrape_per_page_limit = { ctx.lock().unwrap().scrape_per_page };
    let mut repository_collection: Vec<RepositoryCollection> = Vec::new();
    let mut contributor_map: HashMap<String, ContributorCollection> = HashMap::new();
    let mut pull_request_collection: Vec<PullRequestCollection> = Vec::new();

    for target in scrape_targets.into_iter().filter(|t| !t.ignore) {
        let username = &target.username;

        let mut repository: Vec<Repository> = github_client
            .list_repository(&username, scrape_per_page_limit)
            .await
            .map_err(ScrapeError::Github)?;

        // extra filter for Repo target type.
        let repo_target_links: Vec<String> = target.target_links();
        if target.target_type == ScrapeTargetType::Repo {
            if repo_target_links.is_empty() {
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
            // Skip if there isn't any "hacktoberfest" topic on the repository
            if !repo.topics.contains(&"hacktoberfest".into()) {
                continue;
            }

            match scrape_repository_collection(github_client, &username, repo).await {
                Ok(collection) => repository_collection.push(collection),
                Err(ScrapeError::InvalidRepo) => trace!("ignoring {}", repo.full_name),
                Err(e) => log::debug!("err {:?} -> {:?}", e, repo),
            };

            let pulls: Vec<PullRequest> = github_client
                .list_pull_request(&username, &repo.name)
                .await
                .map_err(ScrapeError::Github)?;

            match scrape_contributor_collection(&pulls).await {
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
                Err(ScrapeError::InvalidRepo) => trace!("ignoring {}", repo.full_name),
                Err(e) => log::debug!("err {:?} -> {:?}", e, repo),
            };

            for pull in pulls.iter().filter(|pull| {
                pull.created_at.ge(&FIRST_OCTOBER) && pull.created_at.le(&LAST_OCTOBER)
            }) {
                match scrape_pull_request(github_client, &username, repo, pull.number).await {
                    Ok(pr) => pull_request_collection.push(pr),
                    Err(ScrapeError::InvalidRepo) => trace!("ignoring {}", repo.full_name),
                    Err(e) => log::debug!("err {:?} -> {:?}", e, repo),
                };
            }
        }
    }

    let contributor_collection: Vec<ContributorCollection> =
        contributor_map.into_values().collect();

    let repository_json_collection: String =
        serde_json::to_string(&repository_collection).map_err(ScrapeError::Serde)?;
    let contributor_json_collection: String =
        serde_json::to_string(&contributor_collection).map_err(ScrapeError::Serde)?;
    let pull_request_json_collection: String =
        serde_json::to_string(&pull_request_collection).map_err(ScrapeError::Serde)?;

    {
        let g_ctx = ctx.lock().unwrap();
        let mut _cfg = g_ctx.config.borrow_mut();
        _cfg.scrape_last = Some(Local::now());
        _cfg.cached_map.extend([
            ("repo".into(), repository_json_collection),
            ("contributors".into(), contributor_json_collection),
            ("pull_request".into(), pull_request_json_collection),
        ]);
    }

    Ok(())
}
