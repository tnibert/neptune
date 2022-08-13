mod observer;
mod sprite;
mod graphics;
mod player;
mod input;
mod gameobject;
mod tile;
mod tilemap;
mod collision;
mod game;


use crate::graphics::{convert_renderable};
//use crate::tilearea::{TileArea, create_tile_map};
use crate::game::{Game, SCREEN_WIDTH, SCREEN_HEIGHT};
use crate::gameobject::GameObject;

use std::{thread, time::Duration};

extern crate piston_window;
//use crate::piston_window::Transformed;
use crate::piston_window::RenderEvent;
use crate::piston_window::UpdateEvent;
use piston_window::WindowSettings;

extern crate image as im;

/*
Looking to create a town that the character can move through
and interact with.

- Each renderable item should be able to render itself?
- Renderable items should be represented by a trait?
*/

fn main() {
    let mut game = Game::new();

    let mut window: piston_window::PistonWindow =
        WindowSettings::new("Prototype", [SCREEN_WIDTH, SCREEN_HEIGHT])
        .exit_on_esc(true)
        .vsync(true)
        .build().unwrap();

    let mut texture_context = window.create_texture_context();

    let mut events = piston_window::Events::new(piston_window::EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        // rendering
        if let Some(r) = e.render_args() {

            // clear the window
            window.draw_2d(&e, |c, g, _| {
                piston_window::clear([1.0; 4], g); // Clear to white
            });

            // draw the screen
            window.draw_2d(&e, |c, g, _| {
                //let transform = c.transform.trans(0.0, 0.0);
                let out = convert_renderable(&game.render().unwrap(), &mut texture_context);
                piston_window::image(&out, c.transform, g);
            });
        }

        // game state update
        if let Some(u) = e.update_args() {
            game.update();
        }

        // input handling
        // todo: decouple from piston
        game.input.handle_event(&e);

        // todo: use monotonic clock to find exact time for sleep
        thread::sleep(Duration::new(0, 2_000_000_000u32 / 60));
    }
}

// some notes:
//
// Just remember that 32, Some(32), and None can all be passed into a function whose type implements Into<Option<i32>>.
// This pattern is a relatively easy way to implement optional/default arguments in Rust.
//
// copy_ex() is like copy() but with some more options