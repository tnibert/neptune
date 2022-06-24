//mod observer;
mod sprite;
mod coloredrect;
//mod player;
//mod input;

//use crate::observer;
//use crate::player::Player;
//use crate::input::Input;
//use crate::sprite::Sprite;
use crate::coloredrect::ColoredRect;

//use std::path::Path;
//use std::{thread, time::Duration};

//use std::cell::RefCell;

extern crate piston_window;
use crate::piston_window::PressEvent;
use crate::piston_window::Transformed;
use crate::piston_window::RenderEvent;
use crate::piston_window::UpdateEvent;
use piston_window::WindowSettings;

extern crate image as im;

/// Emulated screen width in pixels
const SCREEN_WIDTH: u32 = 256*2;
/// Emulated screen height in pixels
const SCREEN_HEIGHT: u32 = 240*2;
/// Screen texture size in bytes
//const SCREEN_SIZE: usize = SCREEN_WIDTH * SCREEN_HEIGHT * 3;

const SPEED: f64 = 5.0;

//const SCALE: usize = 1;

fn load_image_asset(fname: &str, texture_context: &mut piston_window::G2dTextureContext) -> piston_window::G2dTexture {
    let img = load_image_asset_buffer(fname);
    return piston_window::Texture::from_image(
            texture_context,
            &img,
            &piston_window::TextureSettings::new()
        ).unwrap();
}

fn load_image_asset_buffer(fname: &str) -> im::ImageBuffer<im::Rgba<u8>,Vec<u8>> {
    let assets = find_folder::Search::Parents(3).for_folder("assets").unwrap();
    let img_path = assets.join(fname);
    let on_top = im::open(img_path).unwrap().into_rgba8();
    let mut img = im::ImageBuffer::from_fn(512, 512, |x, y| {
        if (x + y) % 2 == 0 {
            im::Rgba([0, 0, 0, 0])
        } else {
            im::Rgba([255, 255, 255, 0])
        }
    });

    im::imageops::overlay(&mut img, &on_top, 128, 128);
    //let cropped = im::imageops::crop(&mut img, 0, 0, 30, 40);
    //return cropped.to_image();
    return img;
}

fn main() {
    let mut rect = ColoredRect::new();
    let mut window: piston_window::PistonWindow =
        WindowSettings::new("Prototype", [SCREEN_WIDTH, SCREEN_HEIGHT])
        .exit_on_esc(true)
        .vsync(true)
        .build().unwrap();

    let mut window_size: (f64, f64) = (0.0, 0.0);
    let ss = load_image_asset("reaper.png", &mut window.create_texture_context());

    let mut events = piston_window::Events::new(piston_window::EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        // rendering
        if let Some(r) = e.render_args() {
            window_size = (r.window_size[0] as f64, r.window_size[1] as f64);
            window.draw_2d(&e, |c, g, _| {
                let transform = c.transform.trans(rect.sprite.position[0], rect.sprite.position[1]);

                piston_window::clear([1.0; 4], g); // Clear to white
                /*rectangle(rect.color, // Color
                          rect.sprite.position, // Position/size
                          c.transform, g);*/
                /*fpsfont.draw(&format!("{:.0} FPS", fpscounter.rate()), 
                    &mut glyphs, &c.draw_state,
                    c.transform.trans(10.0, 12.0), // Set the position of the drawing
                    g).unwrap();*/
                piston_window::image(&ss, transform, g);
            });
        }

        // game state update
        if let Some(u) = e.update_args() {
            rect.update(u.dt, window_size);
        }

        // input handling
        if let Some(piston_window::Button::Keyboard(k)) = e.press_args() {
            match k {
                piston_window::Key::Right => {
                    rect.sprite.movespr(SPEED, 0.0);
                },
                piston_window::Key::Left => {
                    rect.sprite.movespr(-SPEED, 0.0);
                },
                piston_window::Key::Down => {
                    rect.sprite.movespr(0.0, SPEED);
                },
                piston_window::Key::Up => {
                    rect.sprite.movespr(0.0, -SPEED);
                }
                _ => {}, // Catch all keys
            }
        }
    }
}

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

// deprecated SDL
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