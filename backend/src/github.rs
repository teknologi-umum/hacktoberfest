use actix_web::http;
use chrono::{DateTime, Utc};
use lazy_static::lazy_static;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client, Response, StatusCode,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{collections::HashMap, fmt};

#[derive(Serialize, Deserialize, Debug)]
pub struct RateLimitResponse {
    resources: Resources,
    rate: Rate,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Rate {
    pub limit: i64,
    pub remaining: i64,
    pub reset: i64,
    pub used: i64,
    pub resource: String,
}

impl Rate {
    fn from_headers(headers: &HeaderMap) -> Self {
        let mut rate = Rate {
            limit: 0,
            remaining: 0,
            reset: 0,
            used: 0,
            resource: "".to_owned(),
        };
        let mut mpairs = vec![
            ("x-ratelimit-limit", &mut rate.limit),
            ("x-ratelimit-remaining", &mut rate.remaining),
            ("x-ratelimit-reset", &mut rate.reset),
            ("x-ratelimit-used", &mut rate.used),
        ];

        for (header_key, v_dst) in mpairs.iter_mut() {
            if let Some(hval_limit) = headers.get(*header_key) {
                let val = hval_limit.to_str().unwrap_or("0");
                if let Ok(v) = val.parse::<i64>() {
                    **v_dst = v;
                }
            }
        }

        // parse resource
        if let Some(hval_resource) = headers.get("x-ratelimit-resource") {
            let val = hval_resource.to_str().unwrap_or("");
            rate.resource = val.into();
        }

        rate
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Resources {
    pub core: Rate,
    pub graphql: Rate,
    pub integration_manifest: Rate,
    pub search: Rate,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Repository {
    pub name: String,
    pub full_name: String,
    pub html_url: String,
    pub description: Option<String>,
    pub language: Option<String>,
    pub stargazers_count: i64,
    pub forks_count: i64,
    pub forks: i64,
    pub topics: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize, Serialize)]
pub struct Issue {
    pub node_id: String,
    pub html_url: String,
    pub title: String,
    pub comments: i64,
    pub user: User,
    pub labels: Vec<Label>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize, Serialize)]
pub struct User {
    pub login: String,
    pub avatar_url: String,
    pub html_url: String,
}

#[derive(Deserialize, Serialize)]
pub struct Label {
    pub name: String,
    pub color: String,
    pub description: String,
}

#[derive(Deserialize, Serialize)]
pub struct PullRequest {
    pub html_url: String,
    pub state: String,
    pub title: String,
    pub number: i64,
    pub locked: bool,
    pub user: User,
    pub merged_at: Option<DateTime<Utc>>,
    pub closed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub merged: Option<bool>,
    pub mergeable_state: Option<String>,
    pub draft: Option<bool>,
    pub requested_reviewers: Option<Vec<User>>,
    pub author_association: Option<String>,
    pub comments: Option<i64>,
    pub review_comments: Option<i64>,
    pub additions: Option<i64>,
    pub deletions: Option<i64>,
    pub changed_files: Option<i64>,
}

pub struct Github {
    client: Client,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GithubErrorResponse {
    pub message: String,
    pub documentation_url: String,
}

lazy_static! {
    pub static ref DEFAULT_CLIENT: Github = Github::new();
}

#[derive(Debug)]
pub struct GithubErrorMetadata {
    pub response: GithubErrorResponse,
    pub rate: Rate,
}

impl GithubErrorMetadata {
    async fn from_http_response(resp: Response) -> Self {
        let rate = Rate::from_headers(resp.headers());
        let response = resp
            .json::<GithubErrorResponse>()
            .await
            .expect("unknown error");
        GithubErrorMetadata { response, rate }
    }
}

#[derive(Debug)]
pub enum GithubError {
    App(GithubErrorMetadata),
    StatusCode(http::StatusCode),
    Request(reqwest::Error),
}

impl std::error::Error for GithubError {}

impl fmt::Display for GithubError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Request(err) => err.fmt(f),
            def => write!(f, "{}", def),
        }
    }
}

impl Github {
    /// Creates a new Github client with no token (limited to 60 requests/hour).
    /// To increase the limit, provide a token and use `new_with_token` instead.
    pub fn new() -> Self {
        Self::new_with_token(None)
    }

    /// Creates a new Github client with an optional
    /// Authorization token (approx. 5000 requests/hour)
    pub fn new_with_token(token: Option<String>) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(
            http::header::USER_AGENT,
            HeaderValue::from_str("hacktoberfest.teknologiumum.com")
                .expect("failed to set User-Agent header"),
        );
        headers.insert(
            http::header::CONTENT_TYPE,
            HeaderValue::from_str("application/vnd.github+json")
                .expect("failed to set Content-Type header"),
        );
        if let Some(token) = token {
            headers.insert(
                http::header::AUTHORIZATION,
                HeaderValue::from_str(&format!("Bearer {token}")[..])
                    .expect("failed to set Authorization header"),
            );
        }

        let client = Client::builder()
            .default_headers(headers)
            .build()
            .expect("Failed to build reqwest::Client");

        Github { client }
    }

    async fn wrap_response<T>(response: Response) -> Result<T, GithubError>
    where
        T: DeserializeOwned + 'static,
    {
        match response.status() {
            StatusCode::OK => response.json::<T>().await.map_err(GithubError::Request),
            StatusCode::FORBIDDEN => Err(GithubError::App(
                GithubErrorMetadata::from_http_response(response).await,
            )),
            status_code => Err(GithubError::StatusCode(status_code)),
        }
    }

    /// Get the rate limit state of the current request client.
    ///
    /// API documentation: https://docs.github.com/en/rest/rate-limit#get-rate-limit-status-for-the-authenticated-user
    pub async fn rate_limit(&self) -> Result<RateLimitResponse, GithubError> {
        let response = self
            .client
            .get("https://api.github.com/rate_limit")
            .send()
            .await
            .map_err(GithubError::Request)?;
        Self::wrap_response(response).await
    }

    /// Lists public repositories for the specified user.
    /// Only shows public repository, sorted by updated, with configurable `per_page` number
    /// of results.
    ///
    /// API documentation: https://docs.github.com/en/rest/repos/repos#list-repositories-for-a-user
    pub async fn list_repository(
        &self,
        user: &String,
        per_page: u8,
    ) -> Result<Vec<Repository>, GithubError> {
        let urlencoded_user = urlencoding::encode(&user[..]);
        let request_url = format!("https://api.github.com/users/{urlencoded_user}/repos");
        let response: Response = self
            .client
            .get(request_url)
            // Figure out what to do with `per_page`? hard coded for now..
            .query(&[
                ("type", "public"),
                ("sort", "updated"),
                ("per_page", &per_page.to_string()[..]),
            ])
            .send()
            .await
            .map_err(GithubError::Request)?;
        Self::wrap_response(response).await
    }

    /// List issues in a repository.
    /// Only returns issues that are considered as an issue (not PRs) by checking their `node_id`
    /// to not be prefixed with "PR_".
    ///
    /// Limited to 30 issues, because it would be too much if we actually show 100 (per the
    /// maximum limit on the documentation).
    ///
    /// API documentation: https://docs.github.com/en/rest/issues/issues#list-repository-issues
    pub async fn list_issues(
        &self,
        user: &String,
        repo: &String,
    ) -> Result<Vec<Issue>, GithubError> {
        let urlencoded_user = urlencoding::encode(&user[..]);
        let urlencoded_repo = urlencoding::encode(&repo[..]);
        let request_url =
            format!("https://api.github.com/repos/{urlencoded_user}/{urlencoded_repo}/issues");
        let response = self
            .client
            .get(request_url)
            .send()
            .await
            .map_err(GithubError::Request)?;
        let resp = Self::wrap_response::<Vec<Issue>>(response).await;
        if let Ok(json_response) = resp {
            let clean_issues = json_response
                .into_iter()
                .filter(|issue| !issue.node_id.starts_with("PR_"))
                .collect();
            return Ok(clean_issues);
        }

        resp
    }

    /// Lists languages for the specified repository. The value shown for each language
    /// is the number of bytes of code written in that language.
    ///
    /// API documentation: https://docs.github.com/en/rest/repos/repos#list-repository-languages
    pub async fn list_languages(
        &self,
        user: &String,
        repo: &String,
    ) -> Result<Vec<String>, GithubError> {
        let urlencoded_user = urlencoding::encode(&user[..]);
        let urlencoded_repo = urlencoding::encode(&repo[..]);
        let request_url =
            format!("https://api.github.com/repos/{urlencoded_user}/{urlencoded_repo}/languages");
        let response = self
            .client
            .get(request_url)
            .send()
            .await
            .map_err(GithubError::Request)?;
        let resp = Self::wrap_response::<HashMap<String, i64>>(response).await;
        if let Ok(json_response) = resp {
            let mut language_set: Vec<(String, i64)> = json_response.into_iter().collect();
            language_set.sort_by(|a, b| {
                let (_, a_bytes) = a;
                let (_, b_bytes) = b;
                b_bytes.cmp(a_bytes)
            });

            let languages = language_set.into_iter().map(|(l, _)| l).collect();
            return Ok(languages);
        }

        Err(resp.err().unwrap())
    }

    /// Lists pull request of a specified repository.
    ///
    /// API documentation: https://docs.github.com/en/rest/pulls/pulls#list-pull-requests
    pub async fn list_pull_request(
        &self,
        user: &String,
        repo: &String,
    ) -> Result<Vec<PullRequest>, GithubError> {
        let urlencoded_user = urlencoding::encode(&user[..]);
        let urlencoded_repo = urlencoding::encode(&repo[..]);
        let request_url =
            format!("https://api.github.com/repos/{urlencoded_user}/{urlencoded_repo}/pulls");

        let response = self
            .client
            .get(request_url)
            .query(&[("per_page", "100"), ("state", "all")])
            .send()
            .await
            .map_err(GithubError::Request)?;

        Self::wrap_response::<Vec<PullRequest>>(response).await
    }

    /// Lists details of a pull request by providing its number.
    ///
    /// The value of the mergeable attribute can be true, false, or null. If the value is null,
    /// then GitHub has started a background job to compute the mergeability. After giving the job
    /// time to complete, resubmit the request. When the job finishes, you will see a non-null
    /// value for the mergeable attribute in the response. If mergeable is true, then
    /// merge_commit_sha will be the SHA of the test merge commit.
    ///
    /// API documentation: https://docs.github.com/en/rest/pulls/pulls#get-a-pull-request
    pub async fn pull_request(
        &self,
        user: &String,
        repo: &String,
        number: i64,
    ) -> Result<PullRequest, GithubError> {
        let urlencoded_user = urlencoding::encode(&user[..]);
        let urlencoded_repo = urlencoding::encode(&repo[..]);
        let request_url = format!(
            "https://api.github.com/repos/{urlencoded_user}/{urlencoded_repo}/pulls/{number}"
        );

        let response = self
            .client
            .get(request_url)
            .send()
            .await
            .map_err(GithubError::Request)?;

        Self::wrap_response::<PullRequest>(response).await
    }
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, Utc};

    use crate::{github::Github, RunContext};

    fn gh_test() -> Github {
        let github_token = RunContext::default().github_token;
        if github_token.is_empty() {
            Github::new()
        } else {
            Github::new_with_token(Some(String::from(github_token)))
        }
    }

    #[test]
    fn test_url_encoding_sec() {
        let name = "aaaa/bb?type=private&_=";
        let p = format!("http://0/asdsdasdad/asaaaaa/{name}/ooookay");
        assert_eq!(
            p,
            "http://0/asdsdasdad/asaaaaa/aaaa/bb?type=private&_=/ooookay"
        );

        let urlencoded_name = urlencoding::encode(&name[..]);
        let p2 = format!("http://0/asdsdasdad/asaaaaa/{urlencoded_name}/ooookay");
        assert_eq!(
            p2,
            "http://0/asdsdasdad/asaaaaa/aaaa%2Fbb%3Ftype%3Dprivate%26_%3D/ooookay"
        );
    }

    #[test]
    fn test_chrono_serde() -> Result<(), String> {
        let tcs = vec![r#""2022-09-21T05:52:31Z""#, "null"];
        for tc in tcs.iter() {
            match serde_json::from_str::<Option<DateTime<Utc>>>(*tc) {
                Ok(dt) => println!("{:?}", dt),
                Err(e) => return Err(e.to_string()),
            }
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_rate_limit() {
        let gh = gh_test();
        let rate_limit = gh.rate_limit().await.unwrap();
        println!("{:?}", rate_limit);
    }

    #[tokio::test]
    async fn test_list_repository() {
        let gh = gh_test();
        let repository = gh
            .list_repository(&"teknologi-umum".into(), 100)
            .await
            .unwrap();
        assert!(!repository.is_empty(), "repository len 0");
    }

    #[tokio::test]
    async fn test_list_repository_user() {
        let gh = gh_test();
        // or just change to anything
        let repo = gh.list_repository(&"ii64".into(), 100).await.unwrap();

        assert!(!repo.is_empty(), "repo len 0");
        println!("{:?}", repo);
    }

    #[tokio::test]
    async fn test_list_issues() {
        let gh = gh_test();
        let issues = gh
            .list_issues(&"teknologi-umum".into(), &"blog".into())
            .await
            .unwrap();
        assert!(!issues.is_empty(), "issues len 0");
    }

    #[tokio::test]
    async fn test_list_languages() {
        let gh = gh_test();
        let languages = gh
            .list_languages(&"teknologi-umum".into(), &"blog".into())
            .await
            .unwrap();
        assert!(!languages.is_empty(), "languages len 0");
        assert_eq!(*languages.get(0).unwrap(), String::from("TypeScript"));
    }

    #[tokio::test]
    async fn test_list_pull_request() {
        let gh = gh_test();
        let pulls = gh
            .list_pull_request(&"teknologi-umum".into(), &"pehape".into())
            .await
            .unwrap();
        assert!(!pulls.is_empty(), "pulls len 0");
    }

    #[tokio::test]
    async fn test_pull_request() {
        let gh = gh_test();
        let pull = gh
            .pull_request(&"teknologi-umum".into(), &"pehape".into(), 1)
            .await
            .unwrap();

        assert_eq!(pull.number, 1);
        assert_eq!(
            pull.html_url,
            "https://github.com/teknologi-umum/pehape/pull/1".to_owned()
        );
        assert_eq!(pull.title, "docs: initialize deadme".to_owned());
        assert!(!pull.draft.unwrap(), "a draft, should not be a draft");
        assert_eq!(pull.mergeable_state.unwrap(), "unknown".to_owned());
        assert!(pull.merged.unwrap(), "not merged, should be merged");
        assert!(!pull.locked, "should not be locked");
        assert_eq!(pull.state, "closed".to_owned());
    }
}
