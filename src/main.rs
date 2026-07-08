use axum::{
    routing::get,
    Router,
};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {

    let app = Router::new()
        .route("/", get(system_status));

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Server pulsing on http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}

async fn system_status() -> &'static str {
    "The backend is awake."
}