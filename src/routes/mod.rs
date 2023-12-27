pub mod hello;
pub mod htmx;
pub mod styles;

#[cfg(test)]
mod route_tests_utils {
    use std::str::from_utf8;

    use askama_axum::Response;
    use axum::body::to_bytes;

    pub async fn into_string(response: Response) -> String {
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        from_utf8(&body).unwrap_or_else(|_| "").parse().unwrap()
    }
}
