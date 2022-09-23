use std::{collections::HashMap};
use actix_web::http;
use chrono::{DateTime, Utc};
use lazy_static::lazy_static;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client, Response,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
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


lazy_static! {
    pub static ref DefaultClient: Github = Github::new();
}

pub struct Github {
    client: Client,
}

impl Github {
    pub fn new() -> Self {
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
        let client = Client::builder()
            .default_headers(headers)
            .build()
            .expect("Failed to build reqwest::Client");

        Github { client }
    }

    /// list_repostory
    /// 
    pub async fn list_repository(&self) -> Result<Vec<Repository>, reqwest::Error> {
        

        let response: Response = self.client
            .get("https://api.github.com/users/teknologi-umum/repos")

            .query(&[
                ("type", "public"),
                ("sort", "updated"),
                ("per_page", "100"),
            ])
            .send()
            .await?;

        let json_response = response.json::<Vec<Repository>>().await?;
        Ok(json_response)
    }

    /// list_issues
    /// 
    pub async fn list_issues(&self, repo: String) -> Result<Vec<Issue>, reqwest::Error> {
        let uencoded_repo = urlencoding::encode(&repo[..]);
        let u = format!("https://api.github.com/repos/teknologi-umum/{uencoded_repo}/issues");
        let response = self
            .client
            .get(u)
            .send()
            .await?;

        let json_response = response.json::<Vec<Issue>>().await?;

        // PR are included on the issues endpoint, we should strip the PRs
        let clean_issues: Vec<Issue> = json_response
            .into_iter()
            .filter(|issue| !issue.node_id.starts_with("PR_"))
            .collect();

        Ok(clean_issues)
    }

    /// list_languages
    /// 
    pub async fn list_languages(&self, repo: String) -> Result<Vec<String>, reqwest::Error> {
        let uencoded_repo = urlencoding::encode(&repo[..]);
        let u = format!("https://api.github.com/repos/teknologi-umum/{uencoded_repo}/languages");
        let response = self
            .client
            .get(u)
            .send()
            .await?;

        let mut json_response = response.json::<HashMap<String, i64>>().await?;

        let mut language_set: Vec<(String, i64)> = vec![];

        for (key, value) in &json_response {
            language_set.push((String::from(key), *value));
        }
        json_response.clear();

        language_set.sort_by(|a, b| {
            let (_, a_bytes) = a;
            let (_, b_bytes) = b;

            b_bytes.cmp(a_bytes)
        });

        let languages: Vec<String> = language_set
            .into_iter()
            .map(|(l, _)| l)
            .collect();

        Ok(languages)
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
        assert!(repository.len() > 0, "repository len 0");
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
