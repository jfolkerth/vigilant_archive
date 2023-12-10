use askama_axum::IntoResponse;

pub async fn clicked() -> impl IntoResponse {
    "Hello, Htmx!"
}

#[cfg(test)]
mod clicked_route_tests {
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use axum::routing::get;
    use axum::Router;
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    use crate::routes::clicked::clicked;

    #[tokio::test]
    async fn clicked_says_hi() {
        let app = Router::new().route("/", get(clicked));
        let request = Request::builder().uri("/").body(Body::empty()).unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(StatusCode::OK, response.status());
        let value = &response.headers()["Content-Type"];
        assert_eq!("text/plain; charset=utf-8", value);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!("Hello, Htmx!", body);
    }
}
