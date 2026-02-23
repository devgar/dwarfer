// src/infrastructure/persistence/sqlite.rs
use crate::domain::model::{ShortUrl, UrlRecord};
use crate::domain::repository::{UrlReader, UrlWriter};
use async_trait::async_trait;
use sqlx::{SqlitePool, Row};

pub struct SqliteRepo {
    pub pool: SqlitePool,
}

#[async_trait]
impl UrlReader for SqliteRepo {
    async fn get_by_id(&self, id: &ShortUrl) -> Result<Option<UrlRecord>, anyhow::Error> {
        let rec = sqlx::query("SELECT id, long_url, created_at, clicks FROM urls WHERE id = ?")
            .bind(id.as_str())
            .fetch_optional(&self.pool)
            .await?;
        if let Some(row) = rec {
            let shorturl = ShortUrl::new(row.get::<String, _>("id")).unwrap();
            let long_url: Option<String> = row.get("long_url");
            let created_at: String = row.get("created_at");
            let created_at = chrono::NaiveDateTime::parse_from_str(&created_at, "%Y-%m-%d %H:%M:%S").unwrap();
            let clicks: i64 = row.get("clicks");
            Ok(Some(UrlRecord { id: shorturl, long_url, created_at, clicks }))
        } else {
            Ok(None)
        }
    }

    async fn track_click(&self, id: &ShortUrl) {
        let _ = sqlx::query("UPDATE urls SET clicks = clicks + 1 WHERE id = ?")
            .bind(id.as_str())
            .execute(&self.pool)
            .await;
    }
}

#[async_trait]
impl UrlWriter for SqliteRepo {
    async fn save(&self, record: &UrlRecord) -> Result<(), anyhow::Error> {
        let _ = sqlx::query("INSERT OR REPLACE INTO urls (id, long_url, created_at, clicks) VALUES (?, ?, ?, ?)")
            .bind(record.id.as_str())
            .bind(record.long_url.as_ref())
            .bind(record.created_at.format("%Y-%m-%d %H:%M:%S").to_string())
            .bind(record.clicks)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn delete(&self, id: &ShortUrl) -> Result<(), anyhow::Error> {
        let _ = sqlx::query("DELETE FROM urls WHERE id = ?")
            .bind(id.as_str())
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn list(&self, limit: usize, offset: usize) -> Result<Vec<UrlRecord>, anyhow::Error> {
        let rows = sqlx::query("SELECT id, long_url, created_at, clicks FROM urls LIMIT ? OFFSET ?")
            .bind(limit as i64)
            .bind(offset as i64)
            .fetch_all(&self.pool)
            .await?;
        Ok(rows.into_iter().filter_map(|row| {
            let shorturl = ShortUrl::new(row.get::<String, _>("id")).ok()?;
            let long_url: Option<String> = row.get("long_url");
            let created_at: String = row.get("created_at");
            let created_at = chrono::NaiveDateTime::parse_from_str(&created_at, "%Y-%m-%d %H:%M:%S").ok()?;
            let clicks: i64 = row.get("clicks");
            Some(UrlRecord { id: shorturl, long_url, created_at, clicks })
        }).collect())
    }
}

// Ejecutor de migraciones
pub async fn run_migrations(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    let _ = sqlx::query(
        "CREATE TABLE IF NOT EXISTS urls (
            id TEXT PRIMARY KEY,
            long_url TEXT,
            created_at TEXT,
            clicks INTEGER DEFAULT 0
        )"
    )
    .execute(pool)
    .await?;
    Ok(())
}
