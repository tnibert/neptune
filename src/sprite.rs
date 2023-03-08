use crate::graphics::*;
use crate::gameobject::GameObject;
use crate::collision::Rect;
use std::collections::HashMap;

// todo: pass these in as parameters or determine from file
// or, encapsulate spritesheet into its own structure
const SPRITE_FRAME_W: usize = 96/3;
const SPRITE_FRAME_H: usize = 144/4;
const SS_DOWN: usize = 4;
const SS_ACROSS: usize = 3;
const MAX_FRAME: usize = SS_ACROSS - 1;

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down
}

fn load_spritesheet(img: &im::RgbaImage, rows: usize, columns: usize) -> HashMap<Direction, Vec<im::RgbaImage>> {
    let mut sheet: HashMap<Direction, Vec<im::RgbaImage>> = HashMap::new();
    let order = [Direction::Down, Direction::Left, Direction::Right, Direction::Up];
    let mut index = 0;
    for y in 0..rows {
        for x in 0..columns {
            //println!("{}, {}", x, y);
            let frame_x_start = x * SPRITE_FRAME_W;
            let frame_y_start = y * SPRITE_FRAME_H;
            let cropped = im::imageops::crop_imm(img, frame_x_start as u32,
                frame_y_start as u32,
                SPRITE_FRAME_W as u32,
                SPRITE_FRAME_H as u32).to_image();
            match sheet.get_mut(&order[index]) {
                Some(vec) => vec.push(cropped),
                None => {
                    sheet.insert(order[index], vec![cropped]);
                }
            };
        }
        index += 1;
    }
    return sheet; 
}

// handles renderable character
pub struct Sprite {
    position: Rect,
    frame: usize,
    frames: HashMap<Direction, Vec<im::RgbaImage>>,
    direction: Direction,
    speed: f64,
    frame_change_count: u32
}

impl Sprite {
    pub fn new(spritesheet_fname: &str, speed: f64, initial_position: Rect) -> Sprite {
        Self {
            position: initial_position,
            frame: 0,
            frames: load_spritesheet(&load_image_asset_buffer(spritesheet_fname), SS_DOWN, SS_ACROSS),
            direction: Direction::Down,
            speed: speed,
            frame_change_count: 0
        }
    }

    pub fn movespr(&mut self, d: Direction) {
        // todo: bounds checking
        match d {
            Direction::Left => {
                self.position.x -= self.speed;
            },
            Direction::Right => {
                self.position.x += self.speed;
            },
            Direction::Up => {
                self.position.y -= self.speed;
            },
            Direction::Down => {
                self.position.y += self.speed;
            },
        }
        
        self.direction = d;
    }

    pub fn current_frame(&self) -> &im::RgbaImage {
        return &self.frames.get(&self.direction).unwrap()[self.frame];
    }
}

impl GameObject for Sprite {
    fn render(&self) -> Option<im::RgbaImage> {
        return Some(self.current_frame().clone());
    }

    fn position(&self) -> Option<Rect> {
        return Some(self.position.clone());
    }

    fn update(&mut self) {
        // update frame
        // todo: beautify this
        if self.frame_change_count > 2 {
            if self.frame < MAX_FRAME {
                self.frame += 1;
            } else {
                self.frame = 0;
            }
            self.frame_change_count = 0;
        }
        self.frame_change_count += 1;
        //println!("{:?}", self.frame);
    }
}