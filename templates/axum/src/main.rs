use axum::{
    routing::{get, post},
    Extension, Router,
};
use std::net::SocketAddr;
use std::sync::Arc;
// use tokio::sync::Mutex;
use tracing::debug;
use tracing_subscriber;

use dotenv::dotenv;
use std::env;

mod application;
mod domain;
mod infrastructure;
mod presentation;

use crate::infrastructure::middleware::database::create_db_pool;
use crate::presentation::application_controller::*;

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = create_db_pool(&database_url).await;

    let app = Router::new()
        .route("/", get(root))
        .route("/_healthcheck", get(healthcheck))
        .layer(Extension(Arc::new(pool)));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
