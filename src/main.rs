use axum::{routing::get, Router};

use crate::routes::hello::clicked;
use crate::routes::hello::hello;
use crate::routes::htmx::htmx;
use crate::routes::styles::css;

mod routes;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello))
        .route("/clicked", get(clicked))
        .route("/static/htmx.min.js", get(htmx))
        .route("/static/styles.css", get(css));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
