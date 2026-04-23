mod api;
mod model;
mod utils;

use axum::{Router, routing::get};

pub struct App {
    router: Router,
}

impl App {
    pub async fn build() -> Self {
        Self {
            router: Router::new()
                .route("/", get(api::get_index))
                .route("/api", get(api::get_short_url))
                .with_state(Self::setup_db_instance().await),
        }
    }

    pub async fn run(self) {
        let listener = tokio::net::TcpListener::bind("localhost:3000")
            .await
            .unwrap();
        println!("Listening on {}", listener.local_addr().unwrap());
        axum::serve(listener, self.router).await.unwrap();
    }

    async fn setup_db_instance() -> sqlx::PgPool {
        let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        sqlx::PgPool::connect(&db_url)
            .await
            .expect("Failed to connect to database")
    }
}
