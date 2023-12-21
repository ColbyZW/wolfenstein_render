extern crate sdl2;
mod gamestate;
mod objects;
mod util;
mod screen;

use std::time::Instant;
use sdl2::render::{TextureCreator, TextureAccess};
use sdl2::video::WindowContext;
use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Scancode};
use gamestate::GameState;
use screen::Screen;


pub const WIDTH: u32 = 1200;
pub const HEIGHT: u32 = 900;
pub const WORLD: u32 = 25;

fn main() {
    let sdl = sdl2::init().unwrap();
    let mut event_pump = sdl.event_pump().unwrap();
    let video_subsystem = sdl.video().unwrap();

    let window = video_subsystem
        .window("Render", WIDTH, HEIGHT)
        .build()
        .unwrap();

    let mut canvas = window
        .into_canvas()
        .target_texture()
        .present_vsync()
        .accelerated()
        .build()
        .unwrap();

    let texture_creator: TextureCreator<WindowContext> = canvas.texture_creator();
    let mut texture = texture_creator.create_texture(
        None, 
        TextureAccess::Target, 
        crate::WIDTH, 
        crate::HEIGHT)
        .expect("Unable to create texture");
    let mut game = GameState::new();
    let mut screen = Screen::new();
    let rot_speed: f32 = 3.0 * 0.016;
    let move_speed: f32 = 3.0 * 0.016;
    let mut fps = Instant::now();
    let mut frames = 0;

    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => break 'main,
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'main,
                _ => {},
            }
        }

        for code in event_pump.keyboard_state().pressed_scancodes() {
            match code {
                Scancode::W => {
                    game.movement(move_speed);
                },
                Scancode::S => {
                    game.movement(-move_speed);
                },
                Scancode::A => {
                    game.rotate(rot_speed);
                },
                Scancode::D => {
                    game.rotate(-rot_speed);
                },
                _ => {}
            }
        }


        screen.render(&mut game);
        texture.update(None, &screen.screen, (crate::WIDTH * 4) as usize).expect("Unable to update texture");
        canvas.copy(&texture, None, None).expect("Unable to copy texture data");

        frames += 1;
        if fps.elapsed().as_secs() >= 1 {
            println!("FPS: {}", frames);
            frames = 0;
            fps = Instant::now();
        }

        canvas.present();
    }

}
