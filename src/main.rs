use axum::response::Html;
use axum::routing::get;
use axum::Router;

#[tokio::main]
async fn main() {
    // let _routes_hello = Router::new().route("/hello", )

    let route_hello: Router<> = Router::new().route(
        "/hello",
        get(|| async { Html("Hello <strong>World!!!</strong>") }),
    );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, route_hello).await.unwrap();
}
