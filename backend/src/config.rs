use anyhow::Result;
use chrono::DateTime;
use core::result::Result::Ok;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Write},
};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum ScrapTargetType {
    User,
    Repo,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ScrapTarget {
    pub username: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub repo_names: Option<Vec<String>>,
    pub target_type: ScrapTargetType,

    // ignore scrap target without removing them from config file
    pub ignore: bool,
}

impl ScrapTarget {
    pub fn user(username: String) -> Self {
        Self {
            username,
            repo_names: None,
            target_type: ScrapTargetType::User,
            ignore: false,
        }
    }
    pub fn repos(username: String, repo_names: Vec<String>) -> Self {
        Self {
            username,
            repo_names: Some(repo_names),
            target_type: ScrapTargetType::Repo,
            ignore: false,
        }
    }

    pub fn ignore(mut self) -> Self {
        self.ignore = true;
        self
    }
    pub fn target_links(&self) -> Vec<String> {
        self.repo_names
            .as_ref()
            .unwrap_or(&Vec::with_capacity(0))
            .into_iter()
            .map(|repo_name| format!("https://github.com/{}/{}", self.username, repo_name))
            .collect::<Vec<String>>()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub scrap_target: Vec<ScrapTarget>,
    pub scrap_last: Option<DateTime<chrono::prelude::Local>>,
    pub cached_map: HashMap<String, String>,
}

impl Config {
    pub fn default() -> Box<Self> {
        Box::new(Self {
            scrap_target: vec![ScrapTarget::user("teknologi-umum".to_owned())],
            scrap_last: None,
            cached_map: HashMap::<String, String>::new(),
        })
    }
    pub fn validate(self) -> Result<Self> {
        Ok(self)
    }
    pub fn load_or_create(path: String) -> Result<Box<Self>> {
        match Self::from_file(&path) {
            Ok(parsed) => Ok(parsed),
            Err(_) => Ok(Self::default().save_yaml_to(&path)?),
        }
    }
    pub fn from_yaml(val: &String) -> Result<Self> {
        Ok(serde_yaml::from_str::<Self>(val)?.validate()?)
    }
    pub fn from_file(path: &String) -> Result<Box<Self>> {
        match File::open(path) {
            Ok(mut handle) => {
                let mut contents = String::new();
                handle.read_to_string(&mut contents)?;
                let parsed = Self::from_yaml(&contents)?;
                contents.clear();
                Ok(Box::new(parsed))
            }
            Err(e) => Err(e.into()),
        }
    }
    pub fn to_string(self: &Box<Self>) -> Result<String> {
        let contents = serde_yaml::to_string(self)?;
        Ok(contents)
    }
    pub fn save_yaml_to(self: Box<Self>, path: &String) -> Result<Box<Self>> {
        let mut handle = File::create(path)?;
        handle.write_all(self.to_string()?.as_bytes())?;
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::config::ScrapTarget;

    use super::Config;

    #[test]
    fn test_scrap_target_ignore() {
        let mut target = ScrapTarget::user("somebody".to_owned());
        assert_eq!(target.ignore, false);
        target = target.ignore();
        assert_eq!(target.ignore, true);
    }

    #[test]
    fn test_scrap_target_links() {
        let targ = ScrapTarget::repos(
            "somebody".to_owned(),
            vec!["a".to_owned(), "b".to_owned(), "c".to_owned()],
        );
        let links = targ.target_links();
        println!("{:?}", links);
    }

    #[test]
    fn test_serde_config() -> anyhow::Result<()> {
        let mut conf = Config::default();
        conf.scrap_target.push(ScrapTarget::repos(
            "somebody".to_owned(),
            vec!["a".to_owned(), "b".to_owned(), "c".to_owned()],
        ));
        conf.scrap_target
            .push(ScrapTarget::user("somebody".to_owned()).ignore());

        let yaml_repr = conf.to_string()?;
        println!("# CHECK\n{yaml_repr}\n---");
        let conf2 = Config::from_yaml(&yaml_repr)?;

        assert!(conf.cached_map.eq(&conf2.cached_map) == true);
        assert!(conf.scrap_last == conf2.scrap_last);

        assert!(
            conf.scrap_target.len()
                == conf
                    .scrap_target
                    .iter()
                    .zip(conf2.scrap_target.iter())
                    .filter(|&(a, b)| a == b)
                    .count()
        );

        Ok(())
    }
}
