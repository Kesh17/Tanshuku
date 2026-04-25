mod api;
mod config;
mod error;
mod model;
mod utils;

use std::sync::Arc;

use axum::{Router, routing::get};

use crate::app::config::Config;

pub struct App {
    router: Router,
    state: AppState,
}

#[derive(Clone)]
struct AppState {
    pub db: sqlx::PgPool,
    pub config: Arc<Config>,
}

impl AppState {
    async fn build() -> Self {
        Self {
            db: setup_db_instance().await,
            config: Arc::new(Config::build()),
        }
    }
}

impl App {
    pub async fn build() -> Self {
        let state = AppState::build().await;
        Self {
            router: Router::new()
                .route("/", get(api::get_index))
                .route("/api", get(api::get_short_url).post(api::set_short_url))
                .with_state(state.clone()),
            state: state,
        }
    }

    pub async fn run(self) {
        let config = self.state.config;
        let listener = tokio::net::TcpListener::bind(&config.get_addr())
            .await
            .unwrap();
        println!("Listening on {}", listener.local_addr().unwrap());
        axum::serve(listener, self.router).await.unwrap();
    }
}

async fn setup_db_instance() -> sqlx::PgPool {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    sqlx::PgPool::connect(&db_url)
        .await
        .expect("Failed to connect to database")
}
