use askama_axum::IntoResponse;
use axum::http::{header, HeaderMap, StatusCode};

pub async fn htmx() -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "text/javascript".parse().unwrap());
    let htmx: &str = include_str!("../../static/htmx.min.js");
    (StatusCode::OK, headers, htmx)
}
