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
