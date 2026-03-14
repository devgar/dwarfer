use crate::domain::auth::{AuthenticationService, Principal, AuthError};
use async_trait::async_trait;

pub struct StaticApiKeyService {
    pub valid_api_key: String,
}

/// Compares two byte slices in constant time to avoid timing attacks
fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.iter().zip(b).fold(0u8, |acc, (x, y)| acc | (x ^ y)) == 0
}

#[async_trait]
impl AuthenticationService for StaticApiKeyService {
    async fn authenticate(&self, token: &str) -> Result<Principal, AuthError> {
        if constant_time_eq(token.as_bytes(), self.valid_api_key.as_bytes()) {
            Ok(Principal {
                api_key: token.to_string(),
                roles: vec!["admin".to_string()],
            })
        } else {
            Err(AuthError::InvalidToken)
        }
    }
}
