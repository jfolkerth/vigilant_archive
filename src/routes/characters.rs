use askama_axum::{IntoResponse, Template};

pub async fn characters() -> impl IntoResponse {
    CharactersTemplate {}
}

#[derive(Template)]
#[template(path = "characters.html")]
struct CharactersTemplate {}

#[cfg(test)]
mod characters_test {
    use askama_axum::IntoResponse;
    use axum::http::{header, StatusCode};

    use crate::routes::characters::characters;
    use crate::routes::route_tests_utils::into_string;

    #[tokio::test]
    async fn characters_has_status_ok() {
        let response = characters().await.into_response();
        assert_eq!(StatusCode::OK, response.status());
    }

    #[tokio::test]
    async fn characters_is_html_content_type() {
        let response = characters().await.into_response();
        let content_type = &response.headers()[header::CONTENT_TYPE];
        assert_eq!("text/html; charset=utf-8", content_type);
    }

    #[tokio::test]
    async fn characters_shows_headers() {
        let response = characters().await.into_response();
        let body = into_string(response).await;
        assert!(body.contains("Name"));
    }

    #[tokio::test]
    async fn characters_lists_each_character() {
        let response = characters().await.into_response();
        let body = into_string(response).await;
        assert!(body.contains("Alice"));
        assert!(body.contains("Blake"));
        assert!(body.contains("Carla"));
    }
}
