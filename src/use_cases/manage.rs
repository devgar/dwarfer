use crate::domain::model::ShortUrl;
use crate::domain::repository::UrlWriter;
use crate::domain::error::DomainError;
use std::sync::Arc;

pub struct ManageUseCase<W: UrlWriter + ?Sized + 'static> {
    writer: Arc<W>,
}

impl<W: UrlWriter + ?Sized + 'static> ManageUseCase<W> {
    pub fn new(writer: Arc<W>) -> Self {
        Self { writer }
    }

    pub async fn delete(&self, id: &ShortUrl) -> Result<(), DomainError> {
        self.writer.delete(id).await.map_err(|_| DomainError::Other("Error al borrar el registro".to_string()))
    }

    pub async fn list(&self, limit: usize, offset: usize) -> Result<Vec<crate::domain::model::UrlRecord>, DomainError> {
        self.writer.list(limit, offset).await.map_err(|_| DomainError::Other("Error al listar registros".to_string()))
    }
}
