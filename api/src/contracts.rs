use serde::Serialize;

#[derive(Serialize)]
pub struct GetMediaResponse {
    media: Vec<Media>
}

impl GetMediaResponse {
    pub fn new(media: Vec<Media>) -> Self {
        Self {
            media
        }
    }
}

#[derive(Serialize)]
pub struct Media {
    id: i32,
    name: String
}

impl Media {
    pub fn new(id: i32, name: String) -> Self {
        Self {
            id,
            name
        }
    }
}
