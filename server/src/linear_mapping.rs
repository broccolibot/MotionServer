use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LinearMapping {
    pub m: f32,
    pub b: f32,
}

impl LinearMapping {
    pub fn new(m: f32, b: f32) -> Self {
        Self { m, b }
    }

    pub fn map(&self, value: f32) -> f32 {
        (value * self.m) + self.b
    }
}

impl Default for LinearMapping {
    fn default() -> Self {
        Self {
            m: 1.0,
            b: 0.0,
        }
    }
}
