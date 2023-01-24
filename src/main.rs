use dotenvy::dotenv;
use dotenvy_macro::dotenv;
use euterpe::init;
#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_uri = dotenv!("DATABASE_URL");
    init(database_uri).await
}
