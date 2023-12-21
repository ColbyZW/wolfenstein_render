#[derive(Copy, Clone)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[derive(Copy, Clone)]
pub struct Vec2U {
    pub x: u32,
    pub y: u32
}

impl Vec2U {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}
