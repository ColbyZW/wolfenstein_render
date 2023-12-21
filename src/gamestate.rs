use crate::util::{Vec2, Vec2U};
use crate::objects::Entity;
use sdl2::pixels::Color;
use std::collections::HashMap;

pub struct GameState {
    pub position: Vec2,
    pub direction: Vec2,
    pub plane: Vec2,
    pub world: [u32; (crate::WORLD * crate::WORLD) as usize],
    pub entities: HashMap<u32, Entity>,
}

impl GameState {
    // Initialize Game State
    pub fn new() -> Self {
        let mut state = GameState {
            position: Vec2::new(12.0, 12.0),
            direction: Vec2::new(-1.0, 0.0),
            plane: Vec2::new(0.0, 0.75),
            world: [0; (crate::WORLD * crate::WORLD) as usize],
            entities: HashMap::new(),
        };

        let wall: Entity = Entity::wall(
            Vec2U::new(0, 0),
            Vec2U::new(0, crate::WORLD - 1),
            Color::RGB(128, 128, 144)
            );
        state.add_entity(wall);
        let wall: Entity = Entity::wall(
            Vec2U::new(0, 0),
            Vec2U::new(crate::WORLD - 1, 0),
            Color::RGB(112, 128, 144)
            );
        state.add_entity(wall);
        let wall: Entity = Entity::wall(
            Vec2U::new(crate::WORLD - 1, 0),
            Vec2U::new(crate::WORLD - 1, crate::WORLD - 1),
            Color::RGB(112, 128, 144)
            );
        state.add_entity(wall);
        let wall: Entity = Entity::wall(
            Vec2U::new(0, crate::WORLD - 1),
            Vec2U::new(crate::WORLD - 1, crate::WORLD - 1),
            Color::RGB(112, 128, 144)
            );
        state.add_entity(wall);

        let wall: Entity = Entity::wall(
            Vec2U::new(3, 5),
            Vec2U::new(7, 5),
            Color::RGB(112, 128, 144)
            );
        state.add_entity(wall);
        
        let wall: Entity = Entity::wall(
            Vec2U::new(3, 6),
            Vec2U::new(3, 9),
            Color::RGB(112, 128, 144)
            );
        state.add_entity(wall);

        let wall: Entity = Entity::wall(
            Vec2U::new(6, 6),
            Vec2U::new(6, 9),
            Color::RGB(112, 128, 144)
            );
        state.add_entity(wall);

        state
    }

    pub fn add_entity(self: &mut Self, entity: Entity) {
        let count = self.entities.len() + 1;

        // The raycaster can only suppport straight line walls
        let start = entity.start;
        let end = entity.end;
        if end.x > start.x {
            for i in start.x..end.x {
                self.world[(i + (crate::WORLD * end.y)) as usize] = count as u32;
            }
        } else {
            for i in start.y..end.y {
                self.world[(start.x + (crate::WORLD * i)) as usize] = count as u32;
            }
        }

        self.entities.insert(count as u32, entity);
    }

    pub fn rotate(self: &mut Self, rot_speed: f32) {
        let old_dir = self.direction;
        let old_plane = self.plane;
        self.direction.x = self.direction.x * rot_speed.cos() - self.direction.y * rot_speed.sin();
        self.direction.y = old_dir.x * rot_speed.sin() + self.direction.y * rot_speed.cos();
        self.plane.x = self.plane.x * rot_speed.cos() - self.plane.y * rot_speed.sin();
        self.plane.y = old_plane.x * rot_speed.sin() + self.plane.y * rot_speed.cos();
    }

    pub fn movement(self: &mut Self, move_speed: f32) {
        self.position.x += self.direction.x * move_speed;
        self.position.y += self.direction.y * move_speed;
    }

}
