
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Image {
    pub id: Uuid,
    pub media_id: Uuid,
    pub between: (u32, u32),
    pub scale: (i32, i32),
    pub crop: (i32, i32, i32, i32),
    pub pos: (u32, u32),
} 

impl Image {

    pub fn new(
        media_id: Uuid,
        between: (u32, u32),
        scale: (i32, i32),
        crop: (i32, i32, i32, i32),
        pos: (u32, u32),
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            media_id,
            between,
            scale,
            crop,
            pos
        }
    }

    // Setters

    pub fn set_between(&mut self, start: u32, duration: u32) -> &mut Self {
        self.between = (start, duration);
        self
    }

    pub fn set_scale(&mut self, w: i32, h: i32) -> &mut Self {
        self.scale = (w, h);
        self
    }

    pub fn set_crop(&mut self, x: i32, y: i32, w: i32, h: i32) -> &mut Self {
        self.crop = (x, y, w, h);
        self
    }

    pub fn set_pos(&mut self, x: u32, y: u32) -> &mut Self {
        self.pos = (x, y);
        self
    }    

}
