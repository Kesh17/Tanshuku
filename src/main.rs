use tanshuku::app::App;

#[tokio::main]
async fn main() {
    App::build().await.run().await
}
