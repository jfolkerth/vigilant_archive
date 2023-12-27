use axum::routing::get;
use axum::Router;

use crate::routes::hello::{clicked, hello};
use crate::routes::htmx::htmx;
use crate::routes::styles::css;

pub fn app() -> Router {
    Router::new()
        .route("/", get(hello))
        .route("/clicked", get(clicked))
        .route("/static/htmx.min.js", get(htmx))
        .route("/static/styles.css", get(css))
}

#[cfg(test)]
mod app_tests {
    use askama_axum::{IntoResponse, Response};
    use axum::body::{Body, Bytes};
    use axum::http::response::Parts;
    use axum::http::Request;
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    use crate::app::app;
    use crate::routes::hello::{clicked, hello};
    use crate::routes::htmx::htmx;
    use crate::routes::styles::css;

    #[tokio::test]
    async fn hello_route() {
        let response = send_request("/").await;
        let expected = hello().await.into_response();
        assert_status_headers_body_match(response, expected).await;
    }

    #[tokio::test]
    async fn clicked_route() {
        let response = send_request("/clicked").await;
        let expected = clicked().await.into_response();
        assert_status_headers_body_match(response, expected).await;
    }

    #[tokio::test]
    async fn static_htmx_route() {
        let response = send_request("/static/htmx.min.js").await;
        let expected = htmx().await.into_response();
        assert_status_headers_body_match(response, expected).await;
    }

    #[tokio::test]
    async fn static_css_route() {
        let response = send_request("/static/styles.css").await;
        let expected = css().await.into_response();
        assert_status_headers_body_match(response, expected).await;
    }

    async fn send_request(route: &str) -> Response {
        let app = app();
        let request = Request::builder().uri(route).body(Body::empty()).unwrap();
        app.oneshot(request).await.unwrap()
    }

    async fn assert_status_headers_body_match(response: Response, expected: Response) {
        let (parts, body) = response.into_parts();
        let (expected_parts, expected_body) = expected.into_parts();
        assert_eq!(expected_parts.status, parts.status);
        assert!(contains_all_headers(parts, expected_parts));
        assert_eq!(to_bytes(expected_body).await, to_bytes(body).await);
    }

    fn contains_all_headers(parts: Parts, expected_parts: Parts) -> bool {
        expected_parts
            .headers
            .keys()
            .all(|key| parts.headers.contains_key(key))
    }

    async fn to_bytes(expected_body: Body) -> Bytes {
        expected_body.collect().await.unwrap().to_bytes()
    }
}
