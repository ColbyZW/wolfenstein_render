use crate::util::Vec2U;
use sdl2::pixels::Color;

#[derive(Copy, Clone)]
pub enum Object {
    WALL
}

#[derive(Copy, Clone)]
pub struct Entity {
    pub kind: Object,
    pub start: Vec2U,
    pub end: Vec2U,
    pub color: Color,
}

impl Entity {
    pub fn wall(start: Vec2U, end: Vec2U, color: Color) -> Self {
        Self {
            kind: Object::WALL,
            start,
            end,
            color
        }
    }
}
