use axum::{
    routing::get,
    Router, http::Method
};
use tower_http::cors::{Any, CorsLayer, self};

mod contracts;
mod handlers;

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(Any);

    let app = Router::new()
        .route("/media", get(handlers::media::get_media))
        .layer(cors);

    println!("Running on port 8000");

    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
