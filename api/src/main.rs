use axum::{
    routing::get,
    Router
};

mod contracts;
mod handlers;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/media", get(handlers::media::get_media));

    println!("Running on port 8000");

    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
