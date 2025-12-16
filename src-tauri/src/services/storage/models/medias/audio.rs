
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Audio {
    pub id: Uuid,
    pub media_id: Uuid,
    pub subclip: Option<(u32, u32)>,
    pub between: Option<(u32, u32)>,
    pub tempo: f64,
    pub volume: f64,
} 

impl Audio {

    pub fn new(
        media_id: Uuid,
        subclip: Option<(u32, u32)>,
        between: Option<(u32, u32)>,
        tempo: Option<f64>,
        volume: Option<f64>
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            media_id,
            subclip,
            between,
            tempo: tempo.unwrap_or(1.0),
            volume: volume.unwrap_or(1.0)
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

    pub fn set_tempo(&mut self, tempo: f64) -> &mut Self {
        self.tempo = tempo;
        self
    }

    pub fn set_volume(&mut self, volume: f64) -> &mut Self {
        self.volume = volume;
        self
    }

}
