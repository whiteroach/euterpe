mod routes;
use std::time::Duration;

use routes::create_routes;
use sea_orm::{ConnectOptions, Database};
use tracing::{log, info};

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

    let app = create_routes(db);
    let addr = "0.0.0.0:8000".parse().unwrap();



    info!("listening on {}", &addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
