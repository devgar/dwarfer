// src/infrastructure/auth/authorization_simple.rs
use crate::domain::auth::{AuthorizationService, Principal, AuthError};
use async_trait::async_trait;

pub struct AllowAllAuthorizationService;

#[async_trait]
impl AuthorizationService for AllowAllAuthorizationService {
    async fn authorize(&self, _principal: &Principal, _action: &str) -> Result<(), AuthError> {
        Ok(()) // Permite todo
    }
}
