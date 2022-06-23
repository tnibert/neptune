//mod observer;
mod sprite;
//mod player;
//mod input;

//use crate::observer;
//use crate::player::Player;
//use crate::input::Input;
use crate::sprite::Sprite;

use std::path::Path;
use std::{thread, time::Duration};

use std::cell::RefCell;

extern crate piston_window;
extern crate find_folder;

use piston_window::*;

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

/// Emulated screen width in pixels
const SCREEN_WIDTH: usize = 256*2;
/// Emulated screen height in pixels
const SCREEN_HEIGHT: usize = 240*2;
/// Screen texture size in bytes
const SCREEN_SIZE: usize = SCREEN_WIDTH * SCREEN_HEIGHT * 3;

const SPEED: f64 = 2.0;

const SCALE: usize = 1;

// omfg this person is a saint:
// https://nora.codes/tutorial/piston-a-game-library-in-rust/

pub struct ColoredRect {
    pub color: [f32; 4],
    //pub position: [f64; 4],
    pub sprite: Sprite
}

impl ColoredRect {
    pub fn new() -> Self {
        ColoredRect {
            color: [1.0, 1.0, 1.0, 1.0],
            //position: [0.0, 0.0, 100.0, 100.0],
            sprite: Sprite::new()
        }
    }

    pub fn update(&mut self, dt: f64, size: (f64, f64)) {
        self.color[0] = Self::update_color(dt as f32, self.color[0], 0.001);
        self.color[1] = Self::update_color(dt as f32, self.color[1], 0.002);
        self.color[2] = Self::update_color(dt as f32, self.color[2], 0.003);
        // X updates
        /*if self.position[0] + self.position[2] >= size.0 ||
            self.position[0] < 0.0 {
            self.velocity[0] = -self.velocity[0];
        }*/
        //self.position[0] += self.velocity[0] * dt * 120.0;

        // Y updates
        /*if self.position[1] + self.position[3] >= size.1 || 
            self.position[1] < 0.0 {
            self.velocity[1] = -self.velocity[1];
        }*/
        //self.position[1] += self.velocity[1] * dt * 120.0;
    }

    fn update_color(dt: f32, color: f32, change: f32)->f32 {
        if color <= 0.0 {
            1.0
        } else {
            color - change * dt * 120.0
        }
    }
}

fn main() {
    let mut rect = ColoredRect::new();
    let mut window: PistonWindow =
        WindowSettings::new("Prototype", [640, 480])
        .exit_on_esc(true)
        .vsync(true)
        .build().unwrap();

    let mut window_size: (f64, f64) = (0.0, 0.0);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        // rendering
        if let Some(r) = e.render_args() {
            window_size = (r.window_size[0] as f64, r.window_size[1] as f64);
            window.draw_2d(&e, |c, g, _| {
                clear([1.0; 4], g); // Clear to white
                rectangle(rect.color, // Color
                          rect.sprite.position, // Position/size
                          c.transform, g);
                /*fpsfont.draw(&format!("{:.0} FPS", fpscounter.rate()), 
                    &mut glyphs, &c.draw_state,
                    c.transform.trans(10.0, 12.0), // Set the position of the drawing
                    g).unwrap();*/
            });
        }

        // game state update
        if let Some(u) = e.update_args() {
            rect.update(u.dt, window_size);
        }

        // input handling
        if let Some(Button::Keyboard(k)) = e.press_args() {
            match k {
                Key::Right => {
                    rect.sprite.movespr(SPEED, 0.0);
                },
                Key::Left => {
                    rect.sprite.movespr(-SPEED, 0.0);
                },
                Key::Down => {
                    rect.sprite.movespr(0.0, SPEED);
                },
                Key::Up => {
                    rect.sprite.movespr(0.0, -SPEED);
                }
                _ => {}, // Catch all keys
            }
        }
    }
}

/*fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow =
        WindowSettings::new("piston: image", [300, 300])
        .exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .unwrap();

    window.set_lazy(true);
    //let window_ref = RefCell::new(window);

    let assets = find_folder::Search::Parents(3).for_folder("assets").unwrap();
    //let assets = find_folder::Search::ParentsThenKids(3, 3)
        //.for_folder("assets").unwrap();
    let ss = assets.join("reaper.png");
    println!("{:?}", ss);
    let ss: G2dTexture = Texture::from_path(
            &mut window.create_texture_context(),
            &ss,
            Flip::None,
            &TextureSettings::new()
        ).unwrap();

    let player = RefCell::new(Player::new());

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _| {
            clear([1.0; 4], g);
            image(&ss, c.transform, g);
        });
    }
}*/

/*
Looking to create a town that the character can move through
and interact with.

A few items outstanding:
- Create Game struct encapsulating SDL window management and game loop
- Implement map with scrolling in four direction
- Keep player on map
- Sprite changing frames while moving
- Subscribe the Player and Game to the Input Observable

- Each renderable item should be able to render itself
*/



/*pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    // why can't I use ? instead of unwrap()?
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG).unwrap();
 
    let window = video_subsystem.window("Prototype",
                                        (SCREEN_WIDTH as usize * SCALE) as u32,
                                        (SCREEN_HEIGHT as usize * SCALE) as u32)
        .position_centered()
        .build()
        .map_err(|e| e.to_string()).unwrap();
 
    let mut canvas = window
        .into_canvas()
        .software()
        .build()
        .map_err(|e| e.to_string()).unwrap();

    // sdl makes texture creator the owner of every texture
    let texture_creator = canvas.texture_creator();

    run(&texture_creator, &canvas, &sdl_context);
}*/

/*pub fn run(/*texture_creator: &TextureCreator<dyn RenderTarget>, 
            canvas: &Canvas<dyn RenderTarget>, sdl_context: &Sdl*/) {
    let ss = texture_creator.load_texture(Path::new("assets/reaper.png")).unwrap();

    let player = RefCell::new(Player::new(&ss));

    let bg_color = Color::RGB(255, 255, 255);
    canvas.set_draw_color(bg_color);
    canvas.clear();
    canvas.present();

    let mut input = Input::new(sdl_context.event_pump().unwrap());
    input.observable.subscribe("up".to_string(), &player);
    input.observable.subscribe("down".to_string(), &player);
    input.observable.subscribe("left".to_string(), &player);
    input.observable.subscribe("right".to_string(), &player);

    'running: loop {
        canvas.set_draw_color(bg_color);
        canvas.clear();
        // handle events
        let stop_signal = input.poll_input();
        if stop_signal {
            break 'running;
        }
        // The rest of the game loop goes here...

        // blit
        render(&mut canvas, bg_color, &player.borrow());

        // todo: use monotonic clock to find exact time for sleep
        thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}*/

// some notes:
//
// Just remember that 32, Some(32), and None can all be passed into a function whose type implements Into<Option<i32>>.
// This pattern is a relatively easy way to implement optional/default arguments in Rust.
//
// copy_ex() is like copy() but with some more options