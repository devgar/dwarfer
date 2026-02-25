// src/main.rs
mod config;
mod error;

mod domain {
    pub mod vobjects {
        pub mod short_url;
    }
    pub mod model;
    pub mod repository;
    pub mod error;
    pub mod auth;
}
mod use_cases {
    pub mod redirect;
    pub mod create;
    pub mod manage;
}
mod infrastructure {
    pub mod auth {
        pub mod apikey;
        pub mod authorization_simple;
    }
    pub mod persistence {
        #[cfg(feature = "repo-sqlite")]
        pub mod sqlite;
        pub mod file;
    }
}
mod presentation {
    #[cfg(feature = "cli")]
    pub mod cli;
    #[cfg(feature = "axum")]
    pub mod http;
}

use std::sync::Arc;
use crate::config::AppConfig;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = AppConfig::load()
        .expect("Couldn't load config");

    // persistence
    let repo: Arc<dyn domain::repository::UrlReader>;
    let writer: Arc<dyn domain::repository::UrlWriter>;

    #[cfg(feature = "repo-sqlite")]
    {
        use crate::infrastructure::persistence::sqlite::SqliteRepo;
        use sqlx::sqlite::SqlitePoolOptions;

        let db_url = config.database_url.as_deref().unwrap_or("sqlite:dwarfer.db?mode=rwc");
        let pool = SqlitePoolOptions::new()
            .connect(db_url)
            .await?;
        let sqlite_repo = Arc::new(SqliteRepo { pool: pool.clone() });
        repo = sqlite_repo.clone();
        writer = sqlite_repo;

        // Si es CLI y el comando es migrate, lo manejamos después o aquí.
        // Pero el CLI parseará sus propios args.
    }

    #[cfg(feature = "repo-file")]
    {
        use crate::infrastructure::persistence::file::JSONFileRepo;
        let json_file_repo = Arc::new(JSONFileRepo::from_config(&config)
            .expect("Couldn't load json file repo"));
        repo  = json_file_repo.clone();
        writer = json_file_repo;
    }
    #[cfg(not(any(feature = "_db", feature = "repo-file")))]
    {
        panic!("Debe activarse al menos una feature de repositorio (ej: repo-sqlite)");
    }

    // Auth
    let api_key = config.api_key.as_deref().unwrap_or("default_api_key");
    let auth_service = Arc::new(crate::infrastructure::auth::apikey::StaticApiKeyService {
        valid_api_key: api_key.to_string(),
    });
    let auth_z_service = Arc::new(crate::infrastructure::auth::authorization_simple::AllowAllAuthorizationService);

    // Casos de uso
    let create_use_case = Arc::new(crate::use_cases::create::CreateUseCase::new(writer.clone(), repo.clone()));
    let redirect_use_case = Arc::new(crate::use_cases::redirect::RedirectUseCase::new(repo.clone()));
    let manage_use_case = Arc::new(crate::use_cases::manage::ManageUseCase::new(writer.clone()));

    #[cfg(feature = "cli")]
    {
        let args: Vec<String> = std::env::args().collect();
        if args.len() > 1 {
            // Nota: En una app real, pasaríamos el pool directamente si es SQLite para migraciones
            #[cfg(feature = "repo-sqlite")]
            {
                use sqlx::sqlite::SqlitePoolOptions;
                let db_url = config.database_url.as_deref().unwrap_or("sqlite:dwarfer.db?mode=rwc");
                let pool = SqlitePoolOptions::new().connect(db_url).await?;
                return presentation::cli::run(args, create_use_case, manage_use_case, &pool).await;
            }
        }
    }

    #[cfg(feature = "axum")]
    {
        let port = 8080;
        println!("Listening API server on port {}...", port);
        presentation::http::run(port, create_use_case, redirect_use_case, manage_use_case, auth_service, auth_z_service).await?;
    }

    Ok(())
}
