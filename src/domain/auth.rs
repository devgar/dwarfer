use async_trait::async_trait;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Principal {
    pub api_key: String,
    pub roles: Vec<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum AuthError {
    InvalidToken,
    Unauthorized,
}

#[async_trait]
pub trait AuthenticationService: Send + Sync {
    async fn authenticate(&self, token: &str) -> Result<Principal, AuthError>;
}

#[async_trait]
pub trait AuthorizationService: Send + Sync {
    async fn authorize(&self, principal: &Principal, action: &str) -> Result<(), AuthError>;
}
