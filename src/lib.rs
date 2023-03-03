mod routes;
use std::time::Duration;
// use axum::Server;
// use hyper::StatusCode;
// use axum::{routing::IntoMakeService, Server, Router, body::Body};
use migration::{Migrator, MigratorTrait};

use routes::create_routes;
use sea_orm::{ConnectOptions, Database};
use tracing::{info, log};

use futures::prelude::*;
use redis::AsyncCommands;

// use hyper::server::conn::AddrIncoming;
// Server<AddrIncoming, IntoMakeService<Router<(),Body>, Exec>>::Output
// Result< <Server<AddrIncoming, IntoMakeService<Router<(), Body>>>as IntoFuture>::Output,hyper::Error>

// async fn database_connector(uri: &str) -> Result<DatabaseConnection,DbErr>{
//     tracing_subscriber::fmt()
//     .with_max_level(tracing::Level::DEBUG)
//     .with_test_writer()
//     .init();
// let mut opt = ConnectOptions::new(uri.to_owned());
// opt.max_connections(100)
//     .min_connections(5)
//     .connect_timeout(Duration::from_secs(8))
//     .acquire_timeout(Duration::from_secs(8))
//     .idle_timeout(Duration::from_secs(8))
//     .max_lifetime(Duration::from_secs(8))
//     .sqlx_logging(true)
//     .sqlx_logging_level(log::LevelFilter::Info);
// // .set_schema_search_path("euterpe_schema".into());
// // let db = Database::connect(opt).await?
//     let db = Database::connect(opt).await?;
//     Ok(db)

// }





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
    // let db = database_connector(database_uri).await.map_err(|e|{ return "Hey".to_owned()})?;

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
