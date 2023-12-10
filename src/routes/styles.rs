use askama_axum::IntoResponse;
use axum::http::{header, HeaderMap, StatusCode};

pub async fn css() -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "text/css".parse().unwrap());
    let stylesheet: &str = include_str!("../../static/styles.css");
    (StatusCode::OK, headers, stylesheet)
}