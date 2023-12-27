use askama_axum::IntoResponse;
use axum::http::{header, HeaderMap, StatusCode};

pub async fn htmx() -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "text/javascript".parse().unwrap());
    let htmx: &str = include_str!("../../static/htmx.min.js");
    (StatusCode::OK, headers, htmx)
}

#[cfg(test)]
mod htmx_tests {
    use askama_axum::IntoResponse;
    use axum::http::{header, StatusCode};

    use crate::routes::htmx::htmx;
    use crate::routes::route_tests_utils::into_string;

    #[tokio::test]
    async fn htmx_has_status_ok() {
        let response = htmx().await.into_response();
        assert_eq!(StatusCode::OK, response.status());
    }

    #[tokio::test]
    async fn htmx_has_javascript_content_type() {
        let response = htmx().await.into_response();
        let content_type = &response.headers()[header::CONTENT_TYPE];
        assert_eq!("text/javascript", content_type);
    }

    #[tokio::test]
    async fn htmx_returns_static_library_file() {
        let response = htmx().await.into_response();
        let htmx: &str = include_str!("../../static/htmx.min.js");
        assert_eq!(htmx, into_string(response).await);
    }
}
