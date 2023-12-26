use crate::routes::hello::{clicked, hello};
use crate::routes::htmx::htmx;
use crate::routes::styles::css;
use axum::routing::get;
use axum::Router;

pub fn router() -> Router {
    Router::new()
        .route("/", get(hello))
        .route("/clicked", get(clicked))
        .route("/static/htmx.min.js", get(htmx))
        .route("/static/styles.css", get(css))
}
