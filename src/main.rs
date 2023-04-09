mod observer;
mod sprite;
mod imgload;
mod player;
mod input;
mod gameobject;
mod tile;
mod tilemap;
mod collision;
mod game;
mod background;

extern crate piston;
extern crate graphics;
extern crate opengl_graphics;
extern crate sdl2_window;
extern crate image as im;

use crate::game::{Game, SCREEN_WIDTH, SCREEN_HEIGHT};
use crate::gameobject::GameObject;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use sdl2_window::Sdl2Window;
use opengl_graphics::*;
use graphics::*;
use std::{thread, time::Duration};

const FRAME_RATE: u32 = 45;

fn main() {
    let mut game = Game::new();

    let opengl = OpenGL::V3_2;
    let mut window: Sdl2Window = WindowSettings::new("Prototype", [SCREEN_WIDTH, SCREEN_HEIGHT])
        .exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .unwrap();

    let mut gl = GlGraphics::new(opengl);
    let mut events = Events::new(EventSettings::new());

    while let Some(e) = events.next(&mut window) {
        // input handling
        // todo: decouple from piston
        game.input.handle_event(&e);

        // game state update
        if let Some(_u) = e.update_args() {
            game.update();
        }

        // rendering
        if let Some(args) = e.render_args() {

            let frame = Texture::from_image(&game.render().unwrap(),
                                       &TextureSettings::new());

            gl.draw(args.viewport(), |c, g| {
                let transform = c.transform.trans(0.0, 0.0);

                clear([1.0; 4], g);
                image(&frame, transform, g);
            });
        }

        // todo:
        // use monotonic clock to find exact time for sleep
        // abstract time tracking
        thread::sleep(Duration::new(0, 1_000_000_000u32 / FRAME_RATE));
    }
}


// some notes:
//
// Just remember that 32, Some(32), and None can all be passed into a function whose type implements Into<Option<i32>>.
// This pattern is a relatively easy way to implement optional/default arguments in Rust.
//
// copy_ex() is like copy() but with some more options
// mem::replace() can swap values of same type
//
// input example: https://github.com/PistonDevelopers/piston-examples/blob/master/examples/user_input/src/main.rs