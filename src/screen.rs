use crate::gamestate::GameState;
use sdl2::pixels::Color;

pub struct Screen {
    pub screen: Vec<u8>,
}

impl Screen {
    pub fn new() -> Self {
        let mut screen = Vec::with_capacity((crate::WIDTH * crate::HEIGHT * 4) as usize);
        screen.resize((crate::WIDTH * crate::HEIGHT * 4) as usize, 0);

        Self {
            screen
        }
    }

    pub fn render(self: &mut Self, state: &mut GameState) {
        let plane = state.plane;
        let camera_dir = state.direction;
        let pos = state.position;
        self.screen = Vec::with_capacity((crate::WIDTH * crate::HEIGHT * 4) as usize);
        self.screen.resize((crate::WIDTH * crate::HEIGHT * 4) as usize, 0);

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

                if state.world[(map_x + (map_y * crate::WORLD as i32)) as usize] != 0 { hit = true };
            }

            let perp_wall_dist = if side == 0 { side_dist_x - delta_x } else { side_dist_y - delta_y };
            let height = (crate::HEIGHT as f32 / perp_wall_dist) as i32;
            let mut start = (crate::HEIGHT / 2) as i32 - (height / 2);
            let mut end = (crate::HEIGHT / 2) as i32 + (height / 2);
            if start < 0 { start = 0; }
            if end >= crate::HEIGHT as i32 { end = crate::HEIGHT as i32 - 1; }

            let entity_id = state.world[(map_x + (map_y * crate::WORLD as i32)) as usize];
            let entity = state.entities.get(&entity_id).expect("No entity found");
            let mut color = entity.color.rgba();

            if side == 1 {
                color = (color.0 / 2, color.1 / 2, color.2 / 2, color.3 / 2);
            }

            self.draw_verline((x*4) as i32, (start) as i32, (end) as i32, color);
        }
    }

    pub fn draw_verline(self: &mut Self, x: i32, start: i32, end: i32, color: (u8,u8,u8,u8)) {
        // Storing pixel data as destructured u32 for faster copying into SDL2 Canvas
        // have to store data in the array as sets of 4 u8's
        for y in start..end {
            self.screen[(x + (y * 4 * crate::WIDTH as i32)) as usize] = color.0;
            self.screen[(x + 1 + (y * 4 * crate::WIDTH as i32)) as usize] = color.1;
            self.screen[(x + 2 + (y * 4 * crate::WIDTH as i32)) as usize] = color.2;
            self.screen[(x + 3 + (y * 4 * crate::WIDTH as i32)) as usize] = color.3;
        }
    }

}
