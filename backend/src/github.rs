use actix_web::error::ParseError::Uri;
use reqwest::{Client, Response};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Repository {
    pub full_name: String,
    pub html_url: String,
    pub description: String,
    #[serde(with = "rfc3339_formatter")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "rfc3339_formatter")]
    pub updated_at: DateTime<Utc>,
    pub language: String,
    pub stargazers_count: i64,
    pub forks_count: i64,
    pub forks: i64,
    pub topics: Vec<String>
}

mod rfc3339_formatter {
    use chrono::{DateTime, Utc, TimeZone};
    use serde::{self, Deserialize, Serializer, Deserializer};

    pub fn serialize<S>(
        date: &DateTime<Utc>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let s = format!("{}", date.to_rfc3339());
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<DateTime<Utc>, D::Error>
        where
            D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Utc.datetime_from_str(&s, "%Y-%m-%dT%H:%M:%SZ").map_err(serde::de::Error::custom)
    }
}

pub async fn list_repository() -> Result<Vec<Repository>, reqwest::Error> {
    let response: Response = Client::new()
        .get("https://api.github.com/users/teknologi-umum/repos?type=public&sort=updated&per_page=100")
        .send()
        .await?;

    let json_response = response.json::<Vec<Repository>>().await?;

    Ok(json_response)
}

pub struct Issue {
    node_id: String,
    html_url: String,
    title: String,
    comments: i64,
    user: User,
    labels: Vec<Label>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>
}

pub struct User {
    login: String,
    avatar_url: String,
    html_url: String
}

pub struct Label {
    name: String,
    color: String,
    description: String
}

pub async fn list_issues(repo: String) -> Result<Vec<Issue>, reqwest::Error> {
    let response = Client::new()
        .get(format!("https://api.github.com/repos/teknologi-umum/{}/issues", repo))
        .send()
        .await?;

    let json_response = response.json::<Vec<Issue>>().await?;

    Ok(json_response)
}

#[cfg(test)]
mod tests {
    use crate::github::{list_repository};

    #[tokio::test]
     async fn test_list_repository() {
        let repository = list_repository().await.unwrap();
        assert_eq!(repository.len(), 10);
    }
}