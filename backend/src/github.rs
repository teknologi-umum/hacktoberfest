use std::{collections::HashMap, fmt};
use actix_web::http;
use chrono::{DateTime, Utc};
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client, Response, StatusCode,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Asd {
    #[serde(rename = "resources")]
    resources: Resources,

    #[serde(rename = "rate")]
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
        let _rate = Rate { limit: todo!(), remaining: todo!(), reset: todo!(), used: todo!(), resource: todo!() };
        let nkey = vec![
            "x-ratelimit-limit",
            "x-ratelimit-remaining",
            "x-ratelimit-reset",
            "x-ratelimit-used",
        ];
        let ndst = vec![
            &_rate.limit,
            &_rate.remaining,
            &_rate.reset,
            &_rate.used,
        ];
        for (i, v) in nkey.iter().enumerate() {
            let vdst = ndst[i];
            if let Some(hval_limit) = headers.get(*v) {
                let val = hval_limit.to_str().unwrap_or("0");
                if let Ok(v) = val.parse::<i64>() {
                    *vdst = v;
                }
            }
        }
        // parse resource
        if let Some(hval_resc) = headers.get("x-ratelimit-resource") {
            let val = hval_resc.to_str().unwrap_or("");
            _rate.resource = val.into();
        }
        
        _rate
    }
}

#[derive(Serialize, Deserialize)]
pub struct Resources {
    pub core: Rate,
    pub graphql: Rate,
    pub integration_manifest: Rate,
    pub search: Rate,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Repository {
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

#[derive(Deserialize)]
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

#[derive(Deserialize)]
pub struct User {
    pub login: String,
    pub avatar_url: String,
    pub html_url: String,
}

#[derive(Deserialize)]
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
    pub messgage: String,
    pub documentation_url: String,
}

#[derive(Debug)]
pub struct GithubErrorMetadata {
    pub response: GithubErrorResponse,
    pub rate: Rate,
}
impl GithubErrorMetadata {
    async fn from_http_respsonse(resp: Response) -> Self {
        let _rate: Rate;
        {
            _rate = Rate::from_headers(resp.headers());
        }
        let response = resp.json::<GithubErrorResponse>().await.expect("unknown error");
        GithubErrorMetadata {
            response,
            rate: _rate,
        }
    }
}

#[derive(Debug)]
pub enum GithubError {
    App(GithubErrorMetadata),
    StatusCode(http::StatusCode),
    Requwuest(reqwest::Error),
}
impl fmt::Display for GithubError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Requwuest(err) => {
                err.fmt(f)
            },
            def => write!(f, stringify!(def)),
        }
    }
}

impl Github {
    pub fn new() -> Self { Self::new_with_token(None) }
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
            let token_value = format!("token {}", token);
            headers.insert(http::header::AUTHORIZATION, HeaderValue::from_str(&token_value[..])
                .expect("failed to set Authorization header"));
        }
        let client = Client::builder()
            .default_headers(headers)
            .build()
            .expect("Failed to build reqwest::Client");
        Github { client }
    }

    /// list_repostory
    /// 
    pub async fn list_repository(&self) -> Result<Vec<Repository>, GithubError> {
        

        let response: Response = self.client
            .get("https://api.github.com/users/teknologi-umum/repos")
            .query(&[
                ("type", "public"),
                ("sort", "updated"),
                ("per_page", "100"),
            ])
            .send()
            .await.map_err(GithubError::Requwuest)?;
        match response.status() {
            StatusCode::OK => {
                let json_response = response.json::<Vec<Repository>>().await
                    .map_err(GithubError::Requwuest);
                return json_response
            },
            StatusCode::FORBIDDEN => {
                return Err(GithubError::App(GithubErrorMetadata::from_http_respsonse(response).await))
            },
            status_code => {
                return Err(GithubError::StatusCode(status_code))
            },
        }
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
            .await.map_err(GithubError::Requwuest)?;
        match response.status() {
            StatusCode::OK => {
                let json_response = response.json::<Vec<Issue>>().await
                    .map_err(GithubError::Requwuest)?;
                let clean_issues = json_response
                    .into_iter()
                    .filter(|iss| !iss.node_id.starts_with("PR_"))
                    .collect();
                return Ok(clean_issues);
            },
            StatusCode::FORBIDDEN => { // For Biden
                return Err(GithubError::App(GithubErrorMetadata::from_http_respsonse(response).await));
            }
            status_code => {
                return Err(GithubError::StatusCode(status_code));
            }
        }
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
            .await.map_err(GithubError::Requwuest)?;
        match response.status() {
            StatusCode::OK => {
                let mut json_response = response.json::<HashMap<String, i64>>().await
                    .map_err(GithubError::Requwuest)?;
                let mut language_set = vec![];
                
                for (key, value) in &json_response {
                    language_set.push((key.into(), *value));
                }
                json_response.clear();

                language_set.sort_by(|a, b| {
                    let (_, a_bytes) = a;
                    let (_, b_bytes) = b;
                    b_bytes.cmp(a_bytes)
                });

                let languages = language_set
                    .into_iter()
                    .map(|(l, _)| l)
                    .collect();
                return Ok(languages);
            },
            StatusCode::FORBIDDEN => {
                return Err(GithubError::App(GithubErrorMetadata::from_http_respsonse(response).await));
            },
            status_code => {
                return Err(GithubError::StatusCode(status_code));
            },
        }
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
        assert_eq!(p, "http://0/asdsdasdad/asaaaaa/aaaa/bb?type=private&_=/ooookay");

        let uencoded_name = urlencoding::encode(&name[..]);
        let p2 = format!("http://0/asdsdasdad/asaaaaa/{uencoded_name}/ooookay");
        assert_eq!(p2, "http://0/asdsdasdad/asaaaaa/aaaa%2Fbb%3Ftype%3Dprivate%26_%3D/ooookay");
    }
    
    #[test]
    fn test_chrono_serde() -> Result<(), String> {
        let tcs = vec![
            r#""2022-09-21T05:52:31Z""#,
            "null",
        ];
        for tc in tcs.iter() {
            match serde_json::from_str::<Option<DateTime<Utc>>>(*tc) {
                Ok(dt) => println!("{:?}", dt),
                Err(e) => return Err(e.to_string()),
            }
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_list_repository() {
        let gh = Github::new();
        let repository = gh.list_repository().await.unwrap();
        assert_eq!(repository.len(), 27);
    }

    #[tokio::test]
    async fn test_list_issues() {
        let gh = Github::new();
        let repository = gh.list_issues("blog".into()).await.unwrap();
        assert!(repository.len() > 0, "repository len 0");
    }

    #[tokio::test]
    async fn test_list_languages() {
        let gh = Github::new();
        let repository =  gh.list_languages(String::from("blog")).await.unwrap();
        assert!(repository.len() > 0, "repositry len 0");
        assert_eq!(*repository.get(0).unwrap(), String::from("TypeScript"));
    }
}
