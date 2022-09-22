use std::collections::HashMap;
use actix_web::http;
use chrono::{DateTime, Utc};
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client, Response,
};
use serde::{Deserialize, Serialize};

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

    pub async fn list_repository(&self) -> Result<Vec<Repository>, reqwest::Error> {
        let response: Response = self.client
            .get("https://api.github.com/users/teknologi-umum/repos?type=public&sort=updated&per_page=100")
            .send()
            .await?;


        let json_response = response.json::<Vec<Repository>>().await?;

        Ok(json_response)
    }

    pub async fn list_issues(&self, repo: String) -> Result<Vec<Issue>, reqwest::Error> {
        let response = self
            .client
            .get(format!(
                "https://api.github.com/repos/teknologi-umum/{repo}/issues"
            ))
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

    pub async fn list_languages(&self, repo: String) -> Result<Vec<String>, reqwest::Error> {
        let response = self
            .client
            .get(format!("https://api.github.com/repos/teknologi-umum/{repo}/languages"))
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
        let rep0 = repository.get(0);
        assert_ne!(rep0, None);
        if let Some(s) = rep0 {
            assert_eq!(*s, String::from("TypeScript"));
        }
    }
}