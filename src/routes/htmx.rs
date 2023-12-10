use askama_axum::IntoResponse;
use axum::http::StatusCode;

pub async fn htmx() -> impl IntoResponse {
    let htmx: &str = include_str!("../../static/htmx.min.js");
    (StatusCode::OK, htmx)
}