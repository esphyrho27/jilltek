use axum::{extract::State, routing::get, Json, Router};
use serde::Serialize;
use sqlx::postgres::PgPoolOptions;
use std::{env, net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::EnvFilter;

#[derive(Clone)]
struct AppState { db: Option<sqlx::PgPool> }
#[derive(Serialize)]
struct Health { status: &'static str, service: &'static str, database: &'static str }

async fn health(State(state): State<Arc<AppState>>) -> Json<Health> {
    let database = match &state.db {
        Some(pool) if sqlx::query("SELECT 1").execute(pool).await.is_ok() => "connected",
        Some(_) => "unavailable",
        None => "not_configured",
    };
    Json(Health { status: "ok", service: "jilltek-api", database })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt().with_env_filter(
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"))
    ).init();
    let db = match env::var("DATABASE_URL") {
        Ok(url) => Some(PgPoolOptions::new().max_connections(10).connect(&url).await?),
        Err(_) => None,
    };
    let state = Arc::new(AppState { db });
    let app = Router::new().route("/health", get(health)).with_state(state)
        .layer(CorsLayer::permissive()) // Restrict to the real frontend origin before production.
        .layer(TraceLayer::new_for_http());
    let host = env::var("API_HOST").unwrap_or_else(|_| "127.0.0.1".into());
    let port: u16 = env::var("API_PORT").unwrap_or_else(|_| "8080".into()).parse()?;
    let addr: SocketAddr = format!("{host}:{port}").parse()?;
    let listener = TcpListener::bind(addr).await?;
    tracing::info!("Jilltek API listening on {addr}");
    axum::serve(listener, app).await?;
    Ok(())
}
