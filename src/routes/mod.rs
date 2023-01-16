use axum::{routing::get, extract::FromRef};
use axum::Router;
use sea_orm::DatabaseConnection;
use tower_http::cors::{Any, CorsLayer};

mod health_check;

use health_check::health_check;
#[derive(Clone, FromRef)]
pub struct AppState {
    database: DatabaseConnection,
}

pub fn create_routes(db:DatabaseConnection) -> Router<()> {
    let cors = CorsLayer::new().allow_origin(Any);
    let app_state = AppState { database: db };

    Router::new().route("/", get(health_check)).layer(cors)
    .with_state(app_state)
}
