// src/presentation/cli.rs
use clap::{Parser, Subcommand};
use crate::use_cases::create::CreateUseCase;
use crate::use_cases::manage::ManageUseCase;
use crate::infrastructure::persistence::sqlite::run_migrations;
use sqlx::SqlitePool;
use std::sync::Arc;

#[derive(Parser)]
#[command(name = "dwarfer")]
#[command(about = "URL Shortener CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Ejecuta las migraciones de la base de datos
    Migrate,
    /// Crea o reclama un ShortUrl
    Create {
        /// URL de destino
        #[arg(short, long)]
        url: String,
        /// ID personalizado (opcional)
        #[arg(short, long)]
        id: Option<String>,
    },
    /// Lista todas las URLs registradas
    List {
        #[arg(short, long, default_value_t = 10)]
        limit: usize,
        #[arg(short, long, default_value_t = 0)]
        offset: usize,
    },
    /// Elimina una URL
    Delete {
        /// ID del ShortUrl
        id: String,
    },
}

pub async fn run<RW, RR>(
    _args: Vec<String>,
    create_use_case: Arc<CreateUseCase<RW, RR>>,
    manage_use_case: Arc<ManageUseCase<RW>>,
    pool: &SqlitePool,
) -> anyhow::Result<()> 
where
    RW: crate::domain::repository::UrlWriter + 'static + ?Sized,
    RR: crate::domain::repository::UrlReader + 'static + ?Sized,
{
    let cli = Cli::parse();

    match cli.command {
        Commands::Migrate => {
            println!("Ejecutando migraciones...");
            run_migrations(pool).await?;
            println!("Migraciones completadas.");
        }
        Commands::Create { url, id } => {
            match create_use_case.create_or_claim(id, url).await {
                Ok(short_id) => println!("ShortUrl creado: {}", short_id.as_str()),
                Err(e) => eprintln!("Error al crear: {}", e),
            }
        }
        Commands::List { limit, offset } => {
            match manage_use_case.list(limit, offset).await {
                Ok(list) => {
                    for rec in list {
                        println!("ID: {:<10} -> URL: {:<30} (Clicks: {})", rec.id.as_str(), rec.long_url.unwrap_or_default(), rec.clicks);
                    }
                }
                Err(e) => eprintln!("Error al listar: {}", e),
            }
        }
        Commands::Delete { id } => {
            let short_id = crate::domain::model::ShortUrl::new(id).expect("ID inválido");
            match manage_use_case.delete(&short_id).await {
                Ok(_) => println!("ShortUrl eliminado."),
                Err(e) => eprintln!("Error al eliminar: {}", e),
            }
        }
    }

    Ok(())
}
