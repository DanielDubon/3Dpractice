pub struct Player {
    pub x: f32,
    pub y: f32,
    pub angle: f32,
}

impl Player {
    pub fn new(x: f32, y: f32, angle: f32) -> Self {
        Player { x, y, angle }
    }
}
