use crate::imgload::*;
use crate::gameobject::GameObject;
use crate::collision::{Rect, convert_world_coord_to_screen_coord};
use crate::game::{SCREEN_WIDTH, SCREEN_HEIGHT};
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
    world_position: Rect,
    frame: usize,
    frames: HashMap<Direction, Vec<im::RgbaImage>>,
    direction: Direction,
    speed: i64,
    frame_change_count: u32,

    // maybe better to split this functionality off into another composed object?
    // there is a consideration of keeping screen and world coordinates in sync
    screen_position: Rect,
    off_screen_allowed: bool,
    viewport: Rect
}

impl Sprite {
    pub fn new(spritesheet_fname: &str, speed: i64, initial_position: Rect, initial_direction: Direction) -> Sprite {
        // todo: base starting viewport from the background provided visible window
        let viewport = Rect{x: 0, y: 0, w: SCREEN_WIDTH as i64, h: SCREEN_HEIGHT as i64};
        let screen_position = convert_world_coord_to_screen_coord(&initial_position, &viewport);

        Self {
            world_position: initial_position,
            screen_position: screen_position,
            frame: 0,
            frames: load_spritesheet(&load_image_asset_buffer(spritesheet_fname), SS_DOWN, SS_ACROSS),
            direction: initial_direction,
            speed: speed,
            frame_change_count: 0,
            off_screen_allowed: false,
            viewport: viewport
        }
    }

    pub fn movespr(&mut self, d: Direction) {
        match d {
            Direction::Left => {
                if self.screen_position.x < 0 && !self.off_screen_allowed {
                    self.world_position.x = 0;
                } else {
                    self.world_position.x -= self.speed;
                }
            },
            Direction::Right => {
                if self.screen_position.x > SCREEN_WIDTH as i64 - self.current_frame().width() as i64 && !self.off_screen_allowed {
                    self.world_position.x = SCREEN_WIDTH as i64 + self.viewport.x - self.current_frame().width() as i64;
                } else {
                    self.world_position.x += self.speed;
                }
            },
            Direction::Up => {
                if self.screen_position.y < 0 && !self.off_screen_allowed {
                    self.world_position.y = 0;
                } else {
                    self.world_position.y -= self.speed;
                }
            },
            Direction::Down => {
                if self.screen_position.y > SCREEN_HEIGHT as i64 - self.current_frame().height() as i64 && !self.off_screen_allowed {
                    self.world_position.y = SCREEN_HEIGHT as i64 + self.viewport.y - self.current_frame().height() as i64;
                } else {
                    self.world_position.y += self.speed;
                }
            },
        }
    }

    pub fn set_viewport(&mut self, viewport: Rect) {
        self.viewport = viewport;
    }

    pub fn current_frame(&self) -> &im::RgbaImage {
        return &self.frames.get(&self.direction).unwrap()[self.frame];
    }

    /* Set the direction the sprite is facing */
    pub fn set_facing(&mut self, d: Direction) {
        self.direction = d;
    }
}

impl GameObject for Sprite {
    fn render(&self) -> Option<im::RgbaImage> {
        return Some(self.current_frame().clone());
    }

    fn position(&self) -> Option<Rect> {
        return Some(self.world_position.clone());
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

        // update screen coordinates
        self.screen_position = convert_world_coord_to_screen_coord(&self.world_position, &self.viewport);
    }
}