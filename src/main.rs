use tanshuku::app::App;

#[tokio::main]
async fn main() {
    App::build().run().await
}
