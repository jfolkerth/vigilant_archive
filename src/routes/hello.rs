use askama_axum::{IntoResponse, Template};

pub async fn hello() -> impl IntoResponse {
    HelloTemplate {
        name: "world",
        route: "/clicked",
    }
}

pub async fn clicked() -> impl IntoResponse {
    HelloTemplate {
        name: "Htmx",
        route: "/",
    }
}

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate<'a, 'b> {
    name: &'a str,
    route: &'b str,
}

#[cfg(test)]
mod clicked_route_tests {
    use std::str::from_utf8;

    use askama_axum::{IntoResponse, Response};
    use axum::body::to_bytes;
    use axum::http::{header, StatusCode};

    use crate::routes::hello::{clicked, hello};

    #[tokio::test]
    async fn hello_has_status_ok() {
        let response = hello().await.into_response();
        assert_eq!(StatusCode::OK, response.status());
    }

    #[tokio::test]
    async fn hello_is_html_content_type() {
        let response = hello().await.into_response();
        let content_type = &response.headers()[header::CONTENT_TYPE];
        assert_eq!("text/html; charset=utf-8", content_type);
    }

    #[tokio::test]
    async fn hello_has_hello_world() {
        let response = hello().await.into_response();
        let body = into_string(response).await;
        assert!(body.contains("Hello, world!"));
    }

    #[tokio::test]
    async fn hello_has_button_that_routes_to_clicked() {
        let response = hello().await.into_response();
        let body = into_string(response).await;
        assert!(body.contains("hx-get=\"/clicked\""));
    }

    #[tokio::test]
    async fn clicked_has_status_ok() {
        let response = clicked().await.into_response();
        assert_eq!(StatusCode::OK, response.status());
    }

    #[tokio::test]
    async fn clicked_is_html_content_type() {
        let response = clicked().await.into_response();
        let content_type = &response.headers()[header::CONTENT_TYPE];
        assert_eq!("text/html; charset=utf-8", content_type);
    }

    #[tokio::test]
    async fn clicked_has_hello_htmx() {
        let response = clicked().await.into_response();
        let body = into_string(response).await;
        assert!(body.contains("Hello, Htmx!"));
    }

    #[tokio::test]
    async fn clicked_has_button_that_routes_to_index() {
        let response = clicked().await.into_response();
        let body = into_string(response).await;
        assert!(body.contains("hx-get=\"/\""));
    }

    async fn into_string(response: Response) -> String {
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        from_utf8(&body).unwrap_or_else(|_| "").parse().unwrap()
    }
}
