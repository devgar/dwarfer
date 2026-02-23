// src/domain/model.rs
use lazy_static::lazy_static;
use regex::Regex;
use std::fmt;
use serde::Deserialize;

lazy_static! {
    static ref SHORTURL_RE: Regex = Regex::new(r"^[a-z0-9.\-_+]{1,64}$").unwrap();
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Deserialize)]
pub struct ShortUrl(String);

impl ShortUrl {
    /// Creates a ShortUrl normalizing input to lowercase and validating its format
    pub fn new<S: Into<String>>(input: S) -> Result<Self, ShortUrlError> {
        let normalized = input.into().to_lowercase();
        if SHORTURL_RE.is_match(&normalized) {
            Ok(ShortUrl(normalized))
        } else {
            Err(ShortUrlError::Invalid)
        }
    }

    pub fn random() -> Self {
        let alphabet: [char; 36] = [
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
            'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7',
            '8', '9',
        ];
        let raw = nanoid::nanoid!(6, &alphabet);
        ShortUrl(raw)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ShortUrlError {
    Invalid,
}

impl fmt::Display for ShortUrlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShortUrlError::Invalid => {
                write!(f, "Invalid ShortUrl: is should contain only a-z, 0-9, . - _ +")
            }
        }
    }
}

impl std::error::Error for ShortUrlError {}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct UrlRecord {
    pub id: ShortUrl,
    pub long_url: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub clicks: i64,
}
