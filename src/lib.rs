mod routes;
use routes::create_routes;

pub async fn init() {
    tracing_subscriber::fmt::init();
    let app = create_routes();
    let addr = "0.0.0.0:8000".parse().unwrap();
    tracing::info!("listening on {}", &addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
