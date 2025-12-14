
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Clip {
    pub start: f64,        
    pub duration: f64,
}

impl Clip {

    pub fn edit_start(&mut self, start: f64) {
        self.start = start;
    }

    pub fn default() -> Self {
        Self {
            start: 0.,
            duration: 10.
        }
    }

    pub fn new(start: f64, duration: f64) -> Self {
        Self {
            start,
            duration
        }
    }
}