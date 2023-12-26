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
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use http_body_util::BodyExt;
    use std::future::Future;
    use tower::ServiceExt;

    use crate::app::app;
    use crate::routes::hello::{clicked, hello};
    use crate::routes::htmx::htmx;
    use crate::routes::styles::css;

    #[tokio::test]
    async fn hello_route() {
        let response = send_request("/").await;
        assert_ok_and_content_type(&response, "text/html; charset=utf-8");
        assert_body_matches(response, hello()).await;
    }

    #[tokio::test]
    async fn clicked_route() {
        let response = send_request("/clicked").await;
        assert_ok_and_content_type(&response, "text/html; charset=utf-8");
        assert_body_matches(response, clicked()).await;
    }

    #[tokio::test]
    async fn static_htmx_route() {
        let response = send_request("/static/htmx.min.js").await;
        assert_ok_and_content_type(&response, "text/javascript");
        assert_body_matches(response, htmx()).await;
    }

    #[tokio::test]
    async fn static_css_route() {
        let response = send_request("/static/styles.css").await;
        assert_ok_and_content_type(&response, "text/css");
        assert_body_matches(response, css()).await;
    }

    async fn send_request(route: &str) -> Response {
        let app = app();
        let request = Request::builder().uri(route).body(Body::empty()).unwrap();
        let response = app.oneshot(request).await.unwrap();
        response
    }

    fn assert_ok_and_content_type(response: &Response, expected_content_type: &str) {
        assert_eq!(response.status(), StatusCode::OK);
        let content_type = &response.headers()["Content-Type"];
        assert_eq!(expected_content_type, content_type);
    }

    async fn assert_body_matches(
        response: Response,
        expected: impl Future<Output = impl IntoResponse + Sized> + Sized,
    ) {
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let expected_body = expected
            .await
            .into_response()
            .into_body()
            .collect()
            .await
            .unwrap()
            .to_bytes();
        assert_eq!(expected_body, body);
    }
}
