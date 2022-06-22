extern crate sdl2;

mod observer;
mod sprite;
mod player;
mod input;

//use crate::observer;
use crate::player::Player;
use crate::input::Input;

/*use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};*/

/*use sdl2::Sdl;
use sdl2::render::{Canvas, WindowCanvas, Texture, TextureCreator, RenderTarget};
use sdl2::rect::{Point, Rect};
//use sdl2::Sdl;
//use sdl2::video::Window;
//use sdl2::video::WindowContext;
use sdl2::pixels::Color;
use sdl2::image::{InitFlag, LoadTexture};*/
use std::path::Path;
use std::{thread, time::Duration};

use std::cell::RefCell;

extern crate piston_window;
extern crate find_folder;

use piston_window::*;

/// Emulated screen width in pixels
const SCREEN_WIDTH: usize = 256*2;
/// Emulated screen height in pixels
const SCREEN_HEIGHT: usize = 240*2;
/// Screen texture size in bytes
const SCREEN_SIZE: usize = SCREEN_WIDTH * SCREEN_HEIGHT * 3;

const SCALE: usize = 1;

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow =
        WindowSettings::new("piston: image", [300, 300])
        .exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .unwrap();

    let assets = find_folder::Search::Parents(3).for_folder("assets").unwrap();
    //let assets = find_folder::Search::ParentsThenKids(3, 3)
        //.for_folder("assets").unwrap();
    let rust_logo = assets.join("reaper.png");
    println!("{:?}", rust_logo);
    let rust_logo: G2dTexture = Texture::from_path(
            &mut window.create_texture_context(),
            &rust_logo,
            Flip::None,
            &TextureSettings::new()
        ).unwrap();
    window.set_lazy(true);
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _| {
            clear([1.0; 4], g);
            image(&rust_logo, c.transform, g);
        });
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

/*fn render(
    canvas: &mut WindowCanvas,
    color: Color,
    player: &Player,
) -> Result<(), String> {
    canvas.set_draw_color(color);
    canvas.clear();

    // get size of window
    let (width, height) = canvas.output_size()?;

    // world coordinate system
    // Treat the center of the screen as the (0, 0) coordinate
    let screen_position = player.spr.position + Point::new(width as i32 / 2, height as i32 / 2);
    let screen_rect = Rect::from_center(screen_position, player.spr.area.width(), player.spr.area.height());

    // why is spritesheet borrowed but area not?
    canvas.copy(&player.spr.spritesheet, player.spr.area, screen_rect)?;

    canvas.present();

    Ok(())
}*/

// winit
/*fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}*/

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