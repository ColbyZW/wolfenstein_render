use crate::util::{Vec2, Vec2U};
use crate::objects::Entity;
use sdl2::pixels::Color;
use std::collections::HashMap;

pub struct GameState {
    position: Vec2,
    direction: Vec2,
    plane: Vec2,
    pub screen: [Color; (crate::WIDTH * crate::HEIGHT) as usize],
    pub world: [u32; (crate::WORLD * crate::WORLD) as usize],
    entities: HashMap<u32, Entity>,
}

impl GameState {
    // Initialize Game State
    pub fn new() -> Self {
        let mut state = GameState {
            position: Vec2::new(12.0, 12.0),
            direction: Vec2::new(-1.0, 0.0),
            plane: Vec2::new(0.0, 0.75),
            screen: [Color::RGB(0,0,0); (crate::WIDTH * crate::HEIGHT) as usize],
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

        // For now we only support walls that are straight lines
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

    pub fn get_pixel_data(self: &mut Self) -> Vec<u8> {
        let pixel_tuple: Vec<(u8, u8, u8, u8)> = self.screen.iter().map(|color| color.rgba()).collect();
        let mut pixels: Vec<u8> = Vec::new();
        for tuple in pixel_tuple {
            pixels.push(tuple.0);
            pixels.push(tuple.1);
            pixels.push(tuple.2);
            pixels.push(tuple.3);
        }

        pixels
    }
    
    pub fn render(self: &mut Self) {
        let plane = self.plane;
        let camera_dir = self.direction;
        let pos = self.position;
        self.screen = [Color::RGB(128, 128, 128); (crate::WIDTH * crate::HEIGHT) as usize];

        for x in 0..crate::WIDTH {
            let cx = (2.0 * (x as f32 / crate::WIDTH as f32)) - 1.0;
            let ray_x = camera_dir.x + plane.x * cx;
            let ray_y = camera_dir.y + plane.y * cx;

            let mut map_x = pos.x as i32;
            let mut map_y = pos.y as i32;

            let mut side_dist_x: f32;
            let mut side_dist_y: f32;

            let delta_x = if ray_x.abs() < 1e-20 { 1e30 } else { (1.0 / ray_x).abs() };
            let delta_y = if ray_y.abs() < 1e-20 { 1e30 } else { (1.0 / ray_y).abs() };


            let step_x: i32;
            let step_y: i32;
            if ray_x < 0.0 {
                step_x = -1;
                side_dist_x = (pos.x - map_x as f32) * delta_x;
            } else {
                step_x = 1;
                side_dist_x = (map_x as f32 + 1.0 - pos.x) * delta_x;
            }

            if ray_y < 0.0 {
                step_y = -1;
                side_dist_y = (pos.y - map_y as f32) * delta_y;
            } else {
                step_y = 1;
                side_dist_y = (map_y as f32 + 1.0 - pos.y) * delta_y;
            }

            let mut hit = false;
            let mut side: i32 = 0;

            while !hit {
                if side_dist_x < side_dist_y {
                    side_dist_x += delta_x;
                    map_x += step_x;
                    side = 0;
                } else {
                    side_dist_y += delta_y;
                    map_y += step_y;
                    side = 1;
                }

                if self.world[(map_x + (map_y * crate::WORLD as i32)) as usize] != 0 { hit = true };
            }

            let perp_wall_dist = if side == 0 { side_dist_x - delta_x } else { side_dist_y - delta_y };
            let height = (crate::HEIGHT as f32 / perp_wall_dist) as i32;
            let mut start = (crate::HEIGHT / 2) as i32 - (height / 2);
            let mut end = (crate::HEIGHT / 2) as i32 + (height / 2);
            if start < 0 { start = 0; }
            if end >= crate::HEIGHT as i32 { end = crate::HEIGHT as i32 - 1; }

            let entity_id = self.world[(map_x + (map_y * crate::WORLD as i32)) as usize];
            let entity = self.entities.get(&entity_id).expect("No entity found");
            let mut color = entity.color;

            if side == 1 {
                let vals: (u8, u8, u8) = color.rgb();
                color = Color::RGB(vals.0 / 2, vals.1 / 2, vals.2 / 2);
            }

            self.draw_verline(x as i32, start as i32, end as i32, color);
        }
    }

    pub fn draw_verline(self: &mut Self, x: i32, start: i32, end: i32, color: Color) {
        for y in start..end {
            self.screen[(x + (y * crate::WIDTH as i32)) as usize] = color;
        }
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
