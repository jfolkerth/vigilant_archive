use askama_axum::IntoResponse;

pub async fn clicked() -> impl IntoResponse {
    "Hello, Htmx!"
}
