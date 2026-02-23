use crate::domain::model::{ShortUrl, UrlRecord};
use crate::domain::repository::{UrlReader, UrlWriter};
use std::fs;
use anyhow::Error;
use async_trait::async_trait;
use serde::Deserialize;
use serde_json;

use crate::config::AppConfig;
use crate::error::AppError;

#[derive(Debug, Deserialize)]
pub struct JSONFileRepo {
    records: Vec<UrlRecord>
}

impl JSONFileRepo {
    pub fn new<S: AsRef<str>>(file_path: S) -> Self {
        println!("Loading file repo {}...", file_path.as_ref());
        let content = fs::read_to_string(file_path.as_ref())
                .expect("Something went wrong reading the file in path");
        let records = serde_json::from_str::<Vec<UrlRecord>>(&content);
        if (records.is_err()) {
            return toml::from_str::<JSONFileRepo>(&content)
                .expect("Something went wrong reading the file in path");
        }
        let records = records.unwrap();
        Self { records }
    }

    pub fn from_config(c: &AppConfig) -> Result<Self, AppError> {
        match &c.repo_file_path {
            Some(path) => Ok(Self::new(path)),
            None => Err(AppError::ConfigMissingError(format!("repo_file_path"))),
        }
    }
}

#[async_trait]
impl UrlReader for JSONFileRepo {
    async fn get_by_id(&self, id: &ShortUrl)-> Result<Option<UrlRecord>, anyhow::Error> {
        Ok(self.records.iter().find(|record| record.id == *id).map(|record| record.clone()))
    }
    async fn track_click(&self, id: &ShortUrl) {}
}

#[async_trait]
impl UrlWriter for JSONFileRepo {
    async fn save(&self, record: &UrlRecord) -> Result<(), Error> {
        panic!("This should not happen")
    }

    async fn delete(&self, id: &ShortUrl) -> Result<(), Error> {
        panic!("This should not happen")
    }

    async fn list(&self, limit: usize, offset: usize) -> Result<Vec<UrlRecord>, Error> {
        panic!("This should not happen");
    }
}