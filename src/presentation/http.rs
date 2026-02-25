use axum::{
    extract::{Path, State},
    http::{StatusCode, HeaderMap},
    response::{IntoResponse, Redirect},
    routing::{get, post, delete},
    Router, Json,
};
use serde::{Deserialize, Serialize};
use crate::use_cases::redirect::RedirectUseCase;
use crate::use_cases::create::CreateUseCase;
use crate::use_cases::manage::ManageUseCase;
use crate::domain::auth::{AuthenticationService, AuthorizationService, Principal};
use crate::domain::repository::{UrlReader, UrlWriter};
use crate::domain::model::ShortUrl;
use std::sync::Arc;
use std::net::SocketAddr;

#[derive(Serialize, Deserialize)]
pub struct CreateRequest {
    pub url: String,
    pub id: Option<String>,
}

pub async fn run<RW, RR, AS, AZ>(
    port: u16,
    create_use_case: Arc<CreateUseCase<RW, RR>>,
    redirect_use_case: Arc<RedirectUseCase<RR>>,
    manage_use_case: Arc<ManageUseCase<RW>>,
    auth_service: Arc<AS>,
    auth_z_service: Arc<AZ>,
) -> anyhow::Result<()>
where
    RW: UrlWriter + 'static + ?Sized,
    RR: UrlReader + 'static + ?Sized,
    AS: AuthenticationService + 'static + ?Sized,
    AZ: AuthorizationService + 'static + ?Sized,
{
    let app_state = AppState {
        create_use_case,
        redirect_use_case,
        manage_use_case,
        auth_service,
        auth_z_service,
    };

    let app = Router::new()
        .route("/:id", get(redirect_handler))
        .route("/", post(create_handler))
        .route("/", get(list_handler))
        .route("/:id", delete(delete_handler))
        .with_state(app_state);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

struct AppState<RW, RR, AS, AZ> 
where
    RW: UrlWriter + 'static + ?Sized,
    RR: UrlReader + 'static + ?Sized,
    AS: AuthenticationService + 'static + ?Sized,
    AZ: AuthorizationService + 'static + ?Sized,
{
    create_use_case: Arc<CreateUseCase<RW, RR>>,
    redirect_use_case: Arc<RedirectUseCase<RR>>,
    manage_use_case: Arc<ManageUseCase<RW>>,
    auth_service: Arc<AS>,
    auth_z_service: Arc<AZ>,
}

impl<RW, RR, AS, AZ> Clone for AppState<RW, RR, AS, AZ>
where
    RW: UrlWriter + 'static + ?Sized,
    RR: UrlReader + 'static + ?Sized,
    AS: AuthenticationService + 'static + ?Sized,
    AZ: AuthorizationService + 'static + ?Sized,
{
    fn clone(&self) -> Self {
        Self {
            create_use_case: self.create_use_case.clone(),
            redirect_use_case: self.redirect_use_case.clone(),
            manage_use_case: self.manage_use_case.clone(),
            auth_service: self.auth_service.clone(),
            auth_z_service: self.auth_z_service.clone(),
        }
    }
}

async fn check_auth<AS, AZ>(
    auth_service: &AS,
    auth_z_service: &AZ,
    headers: &HeaderMap,
    action: &str,
) -> Result<Principal, StatusCode>
where
    AS: AuthenticationService + 'static + ?Sized,
    AZ: AuthorizationService + 'static + ?Sized,
{
    let token = headers
        .get("X-API-KEY")
        .and_then(|v| v.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let principal = auth_service
        .authenticate(token)
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    auth_z_service
        .authorize(&principal, action)
        .await
        .map_err(|_| StatusCode::FORBIDDEN)?;

    Ok(principal)
}

async fn redirect_handler<RW, RR, AS, AZ>(
    State(state): State<AppState<RW, RR, AS, AZ>>,
    Path(id): Path<String>,
) -> impl IntoResponse
where
    RR: UrlReader + 'static + ?Sized,
    RW: UrlWriter + 'static + ?Sized,
    AS: AuthenticationService + 'static + ?Sized,
    AZ: AuthorizationService + 'static + ?Sized,
{
    match ShortUrl::new(id) {
        Ok(short_id) => {
            match state.redirect_use_case.redirect(&short_id).await {
                Ok(long_url) => Redirect::temporary(&long_url).into_response(),
                Err(_) => StatusCode::NOT_FOUND.into_response(),
            }
        }
        Err(_) => StatusCode::BAD_REQUEST.into_response(),
    }
}

async fn create_handler<RW, RR, AS, AZ>(
    State(state): State<AppState<RW, RR, AS, AZ>>,
    headers: HeaderMap,
    Json(payload): Json<CreateRequest>,
) -> impl IntoResponse
where
    RR: UrlReader + 'static + ?Sized,
    RW: UrlWriter + 'static + ?Sized,
    AS: AuthenticationService + 'static + ?Sized,
    AZ: AuthorizationService + 'static + ?Sized,
{
    if let Err(status) = check_auth(&*state.auth_service, &*state.auth_z_service, &headers, "create").await {
        return status.into_response();
    }

    match state.create_use_case.create_or_claim(payload.id, payload.url).await {
        Ok(short_id) => (StatusCode::CREATED, Json(short_id.as_str().to_string())).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}

async fn list_handler<RW, RR, AS, AZ>(
    State(state): State<AppState<RW, RR, AS, AZ>>,
    headers: HeaderMap,
) -> impl IntoResponse
where
    RR: UrlReader + 'static + ?Sized,
    RW: UrlWriter + 'static + ?Sized,
    AS: AuthenticationService + 'static + ?Sized,
    AZ: AuthorizationService + 'static + ?Sized,
{
    if let Err(status) = check_auth(&*state.auth_service, &*state.auth_z_service, &headers, "list").await {
        return status.into_response();
    }

    match state.manage_use_case.list(100, 0).await {
        Ok(list) => (StatusCode::OK, Json(list.iter().map(|r| r.id.as_str().to_string()).collect::<Vec<String>>())).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

async fn delete_handler<RW, RR, AS, AZ>(
    State(state): State<AppState<RW, RR, AS, AZ>>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> impl IntoResponse
where
    RR: UrlReader + 'static + ?Sized,
    RW: UrlWriter + 'static + ?Sized,
    AS: AuthenticationService + 'static + ?Sized,
    AZ: AuthorizationService + 'static + ?Sized,
{
    if let Err(status) = check_auth(&*state.auth_service, &*state.auth_z_service, &headers, "delete").await {
        return status.into_response();
    }

    match ShortUrl::new(id) {
        Ok(short_id) => {
            match state.manage_use_case.delete(&short_id).await {
                Ok(_) => StatusCode::NO_CONTENT.into_response(),
                Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            }
        }
        Err(_) => StatusCode::BAD_REQUEST.into_response(),
    }
}
