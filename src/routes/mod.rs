use axum::Router;
use axum::{extract::FromRef, routing::get};
use redis::Client;
use sea_orm::DatabaseConnection;
use tower_http::cors::{Any, CorsLayer};

mod health_check;

use health_check::health_check;
#[derive(Clone, FromRef)]
pub struct AppState {
    database: DatabaseConnection,
    redis: Client,
}

pub fn create_routes(db: DatabaseConnection, redis: Client) -> Router<()> {
    let cors = CorsLayer::new().allow_origin(Any);
    let app_state = AppState {
        database: db,
        redis,
    };

    Router::new()
        .route("/", get(health_check))
        .layer(cors)
        .with_state(app_state)
}
