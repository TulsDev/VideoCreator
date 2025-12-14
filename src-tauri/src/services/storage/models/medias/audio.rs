
use super::clip::Clip;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Audio {
    // media generics
    id: Uuid,
    media_id: Uuid,
    // clip
    clip: Clip,
    // audio properties
} 

impl Audio {
    pub fn new(media_id: Uuid, _clip: Option<Clip>) -> Self {
        Self {
            id: Uuid::new_v4(),
            media_id,
            clip: {
                let mut r: Clip = Clip::default();
                if let Some(c) = _clip { r = c; }
                r
            }
        }
    }
}
