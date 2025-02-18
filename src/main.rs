use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::Router;

#[tokio::main]
async fn main() {
    let route_hello = Router::new().route(
        "/hello",
        get(handler_hello),
    );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, route_hello).await.unwrap();
}


async fn handler_hello() -> impl IntoResponse {
    println!("->> {:<12} - handler_hello", "HANDLER");
    Html("Hello <strong>World!!!</strong>")
}