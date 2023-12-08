use askama::Template;
use axum::http::{header, HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello))
        .route("/static/styles.css", get(css));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn hello() -> impl IntoResponse {
    HelloTemplate { name: "world" }
}

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate<'a> {
    name: &'a str,
}

async fn css() -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "text/css".parse().unwrap());
    let stylesheet: &str = include_str!("../static/styles.css");
    (StatusCode::OK, headers, stylesheet)
}
