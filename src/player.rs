// src/player.rs

pub struct Player {
    x: f32,
    y: f32,
    angle: f32,
}

impl Player {
    pub fn new(x: f32, y: f32, angle: f32) -> Self {
        Player { x, y, angle }
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn angle(&self) -> f32 {
        self.angle
    }
}
