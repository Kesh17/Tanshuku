use axum::{Router, routing::get};
mod api;

pub struct App {
    router: Router,
}

impl App {
    pub fn build() -> Self {
        Self {
            router: Router::new().route("/", get(api::get_index)),
        }
    }

    pub async fn run(self) {
        let listener = tokio::net::TcpListener::bind("localhost:3000")
            .await
            .unwrap();
        println!("Listening on {}", listener.local_addr().unwrap());
        axum::serve(listener, self.router).await.unwrap();
    }
}
