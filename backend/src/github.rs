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
        let _rate = Rate {
            limit: todo!(),
            remaining: todo!(),
            reset: todo!(),
            used: todo!(),
            resource: todo!(),
        };
        let mpairs = vec![
            ("x-ratelimit-limit", &_rate.limit),
            ("x-ratelimit-remaining", &_rate.remaining),
            ("x-ratelimit-reset", &_rate.reset),
            ("x-ratelimit-used", &_rate.used),
        ];

        for (header_key, mut v_dst) in mpairs.iter() {
            if let Some(hval_limit) = headers.get(*header_key) {
                let val = hval_limit.to_str().unwrap_or("0");
                if let Ok(v) = val.parse::<i64>() {
                    *v_dst = v;
                }
            }
        }

        // parse resource
        if let Some(hval_resource) = headers.get("x-ratelimit-resource") {
            let val = hval_resource.to_str().unwrap_or("");
            _rate.resource = val.into();
        }

        _rate
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
    pub description: String,
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
    Requwest(reqwest::Error),
}
impl std::error::Error for GithubError {}
impl fmt::Display for GithubError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Requwest(err) => err.fmt(f),
            def => write!(f, "{}", def),
        }
    }
}

impl Github {
    pub fn new() -> Self {
        Self::new_with_token(None)
    }
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
            StatusCode::OK => response.json::<T>().await.map_err(GithubError::Requwest),
            StatusCode::FORBIDDEN => Err(GithubError::App(
                GithubErrorMetadata::from_http_response(response).await,
            )),
            status_code => Err(GithubError::StatusCode(status_code)),
        }
    }

    /// rate_limit
    ///
    /// rate limit info
    pub async fn rate_limit(&self) -> Result<RateLimitResponse, GithubError> {
        let response = self
            .client
            .get("https://api.github.com/rate_limit")
            .send()
            .await
            .map_err(GithubError::Requwest)?;
        Self::wrap_response(response).await
    }

    /// list_repostory
    ///
    pub async fn list_repository(&self) -> Result<Vec<Repository>, GithubError> {
        let response: Response = self
            .client
            .get("https://api.github.com/users/teknologi-umum/repos")
            .query(&[("type", "public"), ("sort", "updated"), ("per_page", "100")])
            .send()
            .await
            .map_err(GithubError::Requwest)?;
        Self::wrap_response(response).await
    }

    /// list_issues
    ///
    pub async fn list_issues(&self, repo: String) -> Result<Vec<Issue>, GithubError> {
        let uencoded_repo = urlencoding::encode(&repo[..]);
        let u = format!("https://api.github.com/repos/teknologi-umum/{uencoded_repo}/issues");
        let response = self
            .client
            .get(u)
            .send()
            .await
            .map_err(GithubError::Requwest)?;
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

    /// list_languages
    ///
    pub async fn list_languages(&self, repo: String) -> Result<Vec<String>, GithubError> {
        let uencoded_repo = urlencoding::encode(&repo[..]);
        let u = format!("https://api.github.com/repos/teknologi-umum/{uencoded_repo}/languages");
        let response = self
            .client
            .get(u)
            .send()
            .await
            .map_err(GithubError::Requwest)?;
        let resp = Self::wrap_response::<HashMap<String, i64>>(response).await;
        if let Ok(json_response) = resp {
            let mut language_set: Vec<(String, i64)> = json_response
                .iter()
                .map(|(key, value)| (key.into(), *value))
                .collect();
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
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, Utc};

    use crate::github::Github;

    #[test]
    fn test_url_encoding_sec() {
        let name = "aaaa/bb?type=private&_=";
        let p = format!("http://0/asdsdasdad/asaaaaa/{name}/ooookay");
        assert_eq!(
            p,
            "http://0/asdsdasdad/asaaaaa/aaaa/bb?type=private&_=/ooookay"
        );

        let uencoded_name = urlencoding::encode(&name[..]);
        let p2 = format!("http://0/asdsdasdad/asaaaaa/{uencoded_name}/ooookay");
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
        let gh = Github::new();
        let rate_limit = gh.rate_limit().await.unwrap();
        println!("{:?}", rate_limit);
    }

    #[tokio::test]
    async fn test_list_repository() {
        let gh = Github::new();
        let repository = gh.list_repository().await.unwrap();
        assert!(repository.len() > 0, "repository len 0");
    }

    #[tokio::test]
    async fn test_list_issues() {
        let gh = Github::new();
        let issues = gh.list_issues("blog".into()).await.unwrap();
        assert!(issues.len() > 0, "issues len 0");
    }

    #[tokio::test]
    async fn test_list_languages() {
        let gh = Github::new();
        let languages = gh.list_languages(String::from("blog")).await.unwrap();
        assert!(languages.len() > 0, "languages len 0");
        assert_eq!(*languages.get(0).unwrap(), String::from("TypeScript"));
    }
}
