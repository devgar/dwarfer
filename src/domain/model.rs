// src/domain/model.rs
pub use crate::domain::vobjects::short_url::ShortUrl;
use serde::Deserialize;


#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct UrlRecord {
    pub id: ShortUrl,
    pub long_url: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub clicks: i64,
}
