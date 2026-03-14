use config::{Config, ConfigError};
use std::{env, fs};

pub struct AppConfig {
    pub api_key: Option<String>,
    pub database_url: Option<String>,
    pub repo_file_path: Option<String>,
}

impl AppConfig {
    pub fn load() -> Result<Self, ConfigError> {
        let mut builder = Config::builder()
            .add_source(config::File::with_name("config/default.toml").required(false));

        if let Ok(path) = env::var("DWARFER_CONFIG_PATH") {
            builder = builder.add_source(config::File::with_name(&path).required(true));
        }

        builder = builder.add_source(config::Environment::with_prefix("DWARFER"));

        let config = builder.build()?;

        let mut api_key = config.get_string("api_key").ok();
        if let Ok(api_key_file) = env::var("DWARFER_API_KEY_FILE") {
            if let Ok(contents) = fs::read_to_string(api_key_file) {
                api_key = Some(contents.trim().to_string());
            }
        }

        let mut database_url = None;
        #[cfg(feature = "_db")]
        {
            database_url = config.get_string("database_url").ok();
            if let Ok(db_file) = env::var("DWARFER_DATABASE_URL") {
                if let Ok(contents) = fs::read_to_string(db_file) {
                    database_url = Some(contents.trim().to_string());
                }
            }
        }

        let repo_file_path = {
            #[cfg(feature = "repo-file")]
            {
                env::var("DWARFER_REPO_FILE")
                    .or(config.get_string("repo_file_path")).ok()
            }
            #[cfg(not(feature = "repo-file"))]
            None
        };

        Ok(AppConfig {
            api_key,
            database_url,
            repo_file_path,
        })
    }
}
