mod routes;
use std::time::Duration;
use axum::{response::IntoResponse, http::StatusCode, Json};

use migration::{Migrator, MigratorTrait};

use routes::create_routes;
use sea_orm::{ConnectOptions, Database};
use serde::{Serialize, Deserialize};
use tracing::{info, log};

//use futures::prelude::*;
//use redis::AsyncCommands;

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse<T>{
    pub error: T,
}

#[derive(Debug)]
pub enum EuterpeError {
    DatabaseError(sea_orm::DbErr)
}
impl From<sea_orm::DbErr> for EuterpeError {
    fn from(error: sea_orm::DbErr) -> Self {
        EuterpeError::DatabaseError(error)
    }
}

impl IntoResponse for EuterpeError {
    fn into_response(self) -> axum::response::Response {
        match self {
            EuterpeError::DatabaseError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(
                    ErrorResponse {
                        error: format!("Internal Server Error {}", err)
                    }
                )
            ).into_response()
        }
    }
}


pub async fn init(database_uri: &str) {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();
    let mut opt = ConnectOptions::new(database_uri.to_owned());
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info);
    // .set_schema_search_path("euterpe_schema".into());
    let db = Database::connect(opt).await.unwrap();

    Migrator::up(&db, None).await.unwrap();

    let client_redis = redis::Client::open("redis://127.0.0.1/").unwrap();
    // let client_redis = redis::Client::open("redis://localhost:6379").unwrap();
    //let mut con = client_redis.get_async_connection().await.unwrap();

    // let _ :() = con.set("key1", b"foo").await.unwrap();


    let app = create_routes(db, client_redis);
    let addr = "0.0.0.0:8000".parse().unwrap();
    
    info!("listening on {}", &addr);
    info!("connected to redis instance");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

}
