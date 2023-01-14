use axum::routing::get;
use axum::Router;
use tower_http::cors::{Any, CorsLayer};

mod health_check;

use health_check::health_check;

pub fn create_routes() -> Router<()> {
    let cors = CorsLayer::new().allow_origin(Any);

    Router::new().route("/", get(health_check)).layer(cors)
}
