// src/use_cases/create.rs
use crate::domain::model::{ShortUrl, UrlRecord};
use crate::domain::repository::{UrlReader, UrlWriter};
use crate::domain::error::DomainError;
use std::sync::Arc;
use chrono::Utc;

pub struct CreateUseCase<RW: UrlWriter + ?Sized + 'static, RR: UrlReader + ?Sized + 'static> {
    writer: Arc<RW>,
    reader: Arc<RR>,
}

impl<RW: UrlWriter + ?Sized + 'static, RR: UrlReader + ?Sized + 'static> CreateUseCase<RW, RR> {
    pub fn new(writer: Arc<RW>, reader: Arc<RR>) -> Self {
        Self { writer, reader }
    }

    pub async fn create_or_claim(&self, id_opt: Option<String>, url: String) -> Result<ShortUrl, DomainError> {
        let short_id = match id_opt {
            Some(val) => ShortUrl::new(val).map_err(|_| DomainError::InvalidInput("ShortUrl inválido".to_string()))?,
            None => ShortUrl::random(),
        };

        let existing = self.reader.get_by_id(&short_id).await.map_err(|_| DomainError::Other("Error de repositorio".into()))?;
        match existing {
            Some(record) => {
                if record.long_url.is_none() {
                    // Reclamamos el ID reservado
                    let new_record = UrlRecord {
                        id: short_id.clone(),
                        long_url: Some(url),
                        created_at: record.created_at,
                        clicks: 0,
                    };
                    self.writer.save(&new_record).await.map_err(|_| DomainError::Other("Error al grabar el registro".to_string()))?;
                    Ok(short_id)
                } else {
                    Err(DomainError::Conflict("ShortUrl ya en uso".to_string()))
                }
            }
            None => {
                let now = Utc::now().naive_utc();
                let record = UrlRecord {
                    id: short_id.clone(),
                    long_url: Some(url),
                    created_at: now,
                    clicks: 0,
                };
                self.writer.save(&record).await.map_err(|_| DomainError::Other("Error al grabar el registro".to_string()))?;
                Ok(short_id)
            }
        }
    }
}
