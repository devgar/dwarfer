// src/infrastructure/auth/apikey.rs
use crate::domain::auth::{AuthenticationService, Principal, AuthError};
use async_trait::async_trait;

pub struct StaticApiKeyService {
    pub valid_api_key: String,
}

#[async_trait]
impl AuthenticationService for StaticApiKeyService {
    async fn authenticate(&self, token: &str) -> Result<Principal, AuthError> {
        if token == self.valid_api_key {
            Ok(Principal {
                api_key: token.to_string(),
                roles: vec!["admin".to_string()],
            })
        } else {
            Err(AuthError::InvalidToken)
        }
    }
}
