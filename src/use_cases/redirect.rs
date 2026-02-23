// src/use_cases/redirect.rs
use crate::domain::repository::UrlReader;
use crate::domain::model::ShortUrl;
use crate::domain::error::DomainError;
use std::sync::Arc;

pub struct RedirectUseCase<R: UrlReader + ?Sized + 'static> {
    repo: Arc<R>,
}

impl<R: UrlReader + ?Sized + 'static> RedirectUseCase<R> {
    pub fn new(repo: Arc<R>) -> Self {
        Self { repo }
    }

    pub async fn redirect(&self, short_id: &ShortUrl) -> Result<String, DomainError> {
        let record = self.repo.get_by_id(short_id).await
            .map_err(|_| DomainError::Other("Error de repositorio".into()))?;
        match record {
            Some(r) => {
                if let Some(url) = r.long_url {
                    // Tracking asincrono
                    let repo_clone = self.repo.clone();
                    let short_id_clone = short_id.clone();
                    tokio::spawn(async move {
                        repo_clone.track_click(&short_id_clone).await;
                    });
                    Ok(url)
                } else {
                    Err(DomainError::NotFound)
                }
            }
            None => Err(DomainError::NotFound)
        }
    }
}
