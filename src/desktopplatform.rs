use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use sdl2_window::Sdl2Window;
use opengl_graphics::*;
use graphics::*;
use std::{thread, time::Duration};

use crate::platform::Platform;
use crate::game::{Game, SCREEN_WIDTH, SCREEN_HEIGHT, FRAME_RATE};
use crate::gameobject::GameObject;

/*
 * Abstracts platform for a desktop PC
 * e.g. Linux, Windows, Mac OS - any platform supporting Piston/OpenGL/SDL
 */
pub struct DesktopPlatform {
    window: Sdl2Window,
    gl: GlGraphics,
    events: Events
}

impl DesktopPlatform {
    pub fn new() -> DesktopPlatform {
        let opengl = OpenGL::V3_2;

        Self {
            window: WindowSettings::new("Prototype", [SCREEN_WIDTH, SCREEN_HEIGHT])
                .exit_on_esc(true)
                .graphics_api(opengl)
                .build()
                .unwrap(),
            gl: GlGraphics::new(opengl),
            events: Events::new(EventSettings::new()),
        }
    }
}

impl Platform for DesktopPlatform {
    fn gameloop(&mut self, game: &mut Game) {
        while let Some(e) = self.events.next(&mut self.window) {
            // input handling
            game.input.handle_event(&e);
    
            // game state update
            if let Some(_u) = e.update_args() {
                game.update();
            }
    
            // rendering
            if let Some(args) = e.render_args() {
    
                let frame = Texture::from_image(&game.render().unwrap(),
                                           &TextureSettings::new());
    
                self.gl.draw(args.viewport(), |c, g| {
                    let transform = c.transform.trans(0.0, 0.0);
    
                    clear([1.0; 4], g);
                    image(&frame, transform, g);
                });
            }
    
            // todo:
            // use monotonic clock to find exact time for sleep
            // create abstraction for time tracking (for future no_std)
            thread::sleep(Duration::new(0, 1_000_000_000u32 / FRAME_RATE));
        }
    }
}
