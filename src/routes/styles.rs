use askama_axum::IntoResponse;
use axum::http::{header, HeaderMap, StatusCode};

pub async fn css() -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "text/css".parse().unwrap());
    let stylesheet: &str = include_str!("../../static/styles.css");
    (StatusCode::OK, headers, stylesheet)
}

#[cfg(test)]
mod css_tests {
    use askama_axum::IntoResponse;
    use axum::http::{header, StatusCode};

    use crate::routes::route_tests_utils::into_string;
    use crate::routes::styles::css;

    #[tokio::test]
    async fn css_has_status_ok() {
        let response = css().await.into_response();
        assert_eq!(StatusCode::OK, response.status());
    }

    #[tokio::test]
    async fn css_has_javascript_content_type() {
        let response = css().await.into_response();
        let content_type = &response.headers()[header::CONTENT_TYPE];
        assert_eq!("text/css", content_type);
    }

    #[tokio::test]
    async fn css_returns_static_library_file() {
        let response = css().await.into_response();
        let htmx: &str = include_str!("../../static/styles.css");
        assert_eq!(htmx, into_string(response).await);
    }
}
