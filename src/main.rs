mod observer;
mod sprite;
mod graphics;
mod player;
mod input;

use crate::player::Player;
use crate::input::Input;
use crate::graphics::{convert_renderable};
//use crate::sprite::Direction;

//use std::path::Path;
use std::{thread, time::Duration};

use std::cell::RefCell;

extern crate piston_window;
use crate::piston_window::Transformed;
use crate::piston_window::RenderEvent;
use crate::piston_window::UpdateEvent;
use piston_window::WindowSettings;

extern crate image as im;

/// Emulated screen width in pixels
const SCREEN_WIDTH: u32 = 256*2;
/// Emulated screen height in pixels
const SCREEN_HEIGHT: u32 = 240*2;

/*
Looking to create a town that the character can move through
and interact with.

- Each renderable item should be able to render itself?
- Renderable items should be represented by a trait?
*/

fn main() {
    let player = RefCell::new(Player::new());
    let mut window: piston_window::PistonWindow =
        WindowSettings::new("Prototype", [SCREEN_WIDTH, SCREEN_HEIGHT])
        .exit_on_esc(true)
        .vsync(true)
        .build().unwrap();

    let mut texture_context = window.create_texture_context();
    //let mut window_size: (f64, f64) = (0.0, 0.0);

    let mut input = Input::new();
    input.subscribe(&player, vec!["up", "down", "left", "right", "halt"]);

    let mut events = piston_window::Events::new(piston_window::EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        // rendering
        if let Some(r) = e.render_args() {
            //window_size = (r.window_size[0] as f64, r.window_size[1] as f64);
            window.draw_2d(&e, |c, g, _| {
                let transform = c.transform.trans(player.borrow().spr.position[0], player.borrow().spr.position[1]);
                let ss = convert_renderable(&player.borrow().spr.current_frame(), &mut texture_context);

                piston_window::clear([1.0; 4], g); // Clear to white
                /*fpsfont.draw(&format!("{:.0} FPS", fpscounter.rate()), 
                    &mut glyphs, &c.draw_state,
                    c.transform.trans(10.0, 12.0), // Set the position of the drawing
                    g).unwrap();*/
                piston_window::image(&ss, transform, g);
            });
        }

        // game state update
        if let Some(u) = e.update_args() {
            player.borrow_mut().update();
        }

        // input handling
        input.handle_event(&e);

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