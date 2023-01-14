use axum::Json;

pub async fn health_check() -> Json<String> {
    Json("health_check".into())
}
