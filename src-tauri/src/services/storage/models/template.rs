
use super::medias::{
    video::Video,
    audio::Audio
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Template {
    videos: HashMap<Uuid,Video>,
    audios: HashMap<Uuid,Audio>,
    // images: HashMap<Uuid,Images>,
}

impl Template {

    pub fn add_video(&mut self, video: Video) -> Uuid {
        let id = video.id;
        self.videos.insert(id, video);
        id
    }

    pub fn get_video(&mut self, id: Uuid) -> Option<&mut Video> {
        self.videos.get_mut(&id)
    }

    pub fn del_video(&mut self, id: Uuid)-> Option<Video> {
        self.videos.remove(&id)
    }

    pub fn new() -> Self {
        Self {
            videos: HashMap::new(),
            audios: HashMap::new(),
            images: HashMap::new()
        }
    }
}
