use sdl2::render::Texture;
use sdl2::rect::{Point, Rect};

// handles renderable character
//#[derive(Debug)]
pub struct Sprite<'a> {
    pub position: Point,
    pub area: Rect,
    pub speed: i32,
    pub spritesheet: Texture<'a>
}

impl <'a> Sprite<'a> {
    pub fn new(width: u32, height: u32, speed: i32, spritesheet: Texture<'a>) -> Sprite {
        Self {
            position: Point::new(0, 0),
            // src position in the spritesheet
            area: Rect::new(0, 0, width, height),
            speed: speed,
            spritesheet: spritesheet
        }
    }

    pub fn movespr(&mut self, xdiff: i32, ydiff: i32) {
        self.position = self.position.offset(xdiff, ydiff);
    }
}