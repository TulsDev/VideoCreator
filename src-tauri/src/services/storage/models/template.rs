
use super::medias::video::Video;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Template {
    videos: Vec<Video>,
    audios: Vec<Audio>,
    images: Vec<Images>,
}

impl Template {
    pub fn new() -> Self {
        Self {
            videos: vec![]
        }
    }
}
