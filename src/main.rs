use tanshuku::app::App;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    App::build().await.run().await
}
