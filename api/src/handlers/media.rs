use axum::{http::StatusCode, Json};

use crate::contracts::{Media, GetMediaResponse};

pub async fn get_media() -> (StatusCode, Json<GetMediaResponse>) {
    let media = vec![
        Media::new(0, String::from("shaba")),
        Media::new(1, String::from("shaba 2: the revenge")),
        Media::new(2, String::from("shaba 3: a new hope"))
    ];
    let response = GetMediaResponse::new(media);
    (StatusCode::OK, Json(response))
}
