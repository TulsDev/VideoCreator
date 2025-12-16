
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Video {
    pub id: Uuid,
    pub media_id: Uuid,
    pub subclip: Option<(u32, u32)>,
    pub between: Option<(u32, u32)>,
    pub scale: Option<(i32, i32)>,
    pub crop: Option<(i32, i32, i32, i32)>,
    pub pos: (u32, u32),
    pub speed: f64,
    pub fps: u32,
} 

impl Video {

    pub fn new(
        media_id: Uuid,
        subclip: Option<(u32, u32)>,
        between: Option<(u32, u32)>,
        scale: Option<(i32, i32)>,
        crop: Option<(i32, i32, i32, i32)>,
        pos: Option<(u32, u32)>,
        speed: Option<f64>,
        fps: Option<u32>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            media_id,
            subclip,
            between,
            scale,
            crop,
            pos: pos.unwrap_or((0, 0)),
            speed: speed.unwrap_or(1.),
            fps: fps.unwrap_or(30),
        }
    }

    // Setters

    pub fn set_subclip(&mut self, start: u32, duration: u32) -> &mut Self {
        self.subclip = Some((start, duration));
        self
    }

    pub fn set_between(&mut self, start: u32, duration: u32) -> &mut Self {
        self.between = Some((start, duration));
        self
    }

    pub fn set_scale(&mut self, w: i32, h: i32) -> &mut Self {
        self.scale = Some((w, h));
        self
    }

    pub fn set_crop(&mut self, x: i32, y: i32, w: i32, h: i32) -> &mut Self {
        self.crop = Some((x, y, w, h));
        self
    }

    pub fn set_pos(&mut self, x: u32, y: u32) -> &mut Self {
        self.pos = (x, y);
        self
    }

    pub fn set_speed(&mut self, speed: f64) -> &mut Self {
        self.speed = speed; 
        self
    }

    pub fn set_fps(&mut self, fps: u32) -> &mut Self {
        self.fps = fps.max(1);
        self
    }

}
