use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let port = "3000";
    println!("Listening on http://localhost:{}", port);

    let app = Router::new().route("/", get(handler));

    let addr = format!("0.0.0.0:{}", port);
    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
async fn handler() -> &'static str {
    println!("new request!");
    "ok!"
}
