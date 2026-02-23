// src/domain/repository.rs
use super::model::{ShortUrl, UrlRecord};
use async_trait::async_trait;

/// Trait minimalista para nodos de redirección
#[async_trait]
pub trait UrlReader: Send + Sync {
    async fn get_by_id(&self, id: &ShortUrl) -> Result<Option<UrlRecord>, anyhow::Error>;
    async fn track_click(&self, id: &ShortUrl);
}

/// Trait completo para gestión y CRUD
#[async_trait]
pub trait UrlWriter: Send + Sync {
    async fn save(&self, record: &UrlRecord) -> Result<(), anyhow::Error>;
    async fn delete(&self, id: &ShortUrl) -> Result<(), anyhow::Error>;
    async fn list(&self, limit: usize, offset: usize) -> Result<Vec<UrlRecord>, anyhow::Error>;
}
