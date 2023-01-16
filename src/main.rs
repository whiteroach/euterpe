use euterpe::init;
use dotenvy::dotenv;
use dotenvy_macro::dotenv;
#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_uri = dotenv!("DATABASE_URL");
    init(database_uri).await
}

