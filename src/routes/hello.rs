use askama_axum::{IntoResponse, Template};

pub async fn hello() -> impl IntoResponse {
    HelloTemplate { name: "world" }
}

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate<'a> {
    name: &'a str,
}
