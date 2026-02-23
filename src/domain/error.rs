// src/domain/error.rs
use std::fmt;

#[derive(Debug)]
pub enum DomainError {
    NotFound,
    InvalidInput(String),
    Unauthorized,
    Conflict(String),
    Other(String),
}

impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DomainError::NotFound => write!(f, "No encontrado"),
            DomainError::InvalidInput(msg) => write!(f, "Entrada inválida: {}", msg),
            DomainError::Unauthorized => write!(f, "Acceso no autorizado"),
            DomainError::Conflict(msg) => write!(f, "Conflicto: {}", msg),
            DomainError::Other(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for DomainError {}
