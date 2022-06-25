use crate::graphics::*;
use std::collections::HashMap;

// todo: pass these in as parameters or determine from file
const SPRITE_FRAME_W: usize = 96/3;
const SPRITE_FRAME_H: usize = 144/4;
const SS_DOWN: usize = 4;
const SS_ACROSS: usize = 3;
const MAX_FRAME: usize = SS_ACROSS - 1;
const SPEED: f64 = 5.0;

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down
}

fn load_spritesheet(img: &im::RgbaImage) -> HashMap<Direction, Vec<im::RgbaImage>> {
    let mut sheet: HashMap<Direction, Vec<im::RgbaImage>> = HashMap::new();
    //let mut sheet = [[im::RgbaImage::new(); SS_DOWN]; SS_ACROSS];
    //state[0][1] = 42;
    let order = [Direction::Down, Direction::Left, Direction::Right, Direction::Up];
    let mut index = 0;
    for y in 0..SS_DOWN {
        for x in 0..SS_ACROSS {
            println!("{}, {}", x, y);
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
//#[derive(Debug)]
pub struct Sprite {
    pub position: [f64; 4],
    frame: usize,
    //pub spritesheet: im::RgbaImage,
    frames: HashMap<Direction, Vec<im::RgbaImage>>,
    direction: Direction
    //pub area: Rect,
    //pub speed: i32,
}

impl Sprite {
    pub fn new() -> Sprite {
        //let ss = load_image_asset_buffer("reaper.png");
        Self {
            position: [0.0, 0.0, 100.0, 100.0],
            frame: 0,
            //spritesheet: ss,
            // todo: pass in filename
            frames: load_spritesheet(&load_image_asset_buffer("reaper.png")),
            direction: Direction::Down
            // src position in the spritesheet
            //area: Rect::new(0, 0, width, height),
            //speed: speed,
            //spritesheet: load_sprite_img(&window)
        }
    }

    pub fn movespr(&mut self, d: Direction) {
        // todo: bounds checking
        match d {
            Direction::Left => {
                self.position[0] -= SPEED;
            },
            Direction::Right => {
                self.position[0] += SPEED;
            },
            Direction::Up => {
                self.position[1] -= SPEED;
            },
            Direction::Down => {
                self.position[1] += SPEED;
            },
        }
        
        println!("{:?}", self.position);
        self.direction = d;

        if self.frame < MAX_FRAME {
            self.frame += 1;
        } else {
            self.frame = 0;
        }
        println!("{:?}", self.frame);
    }

    pub fn current_frame(&self) -> &im::RgbaImage {
        return &self.frames.get(&self.direction).unwrap()[self.frame];
    }
}