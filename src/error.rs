use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Required {0} config is missing")]
    ConfigMissingError(String),
    #[error("Required repo file config is missing")]
    ConfigRepoFileMissingError,
}