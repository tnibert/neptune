use crate::sprite::Sprite;
use crate::sprite::Direction;
use crate::observer::Observer;
use crate::observer::Event;

//use std::cell::RefCell;

//const PLAYER_W: u32 = 26;
//const PLAYER_H: u32 = 36;
//const PLAYER_SPEED: i32 = 5;

// handles player, received input
//#[derive(Debug)]
pub struct Player {
    // todo: make not pub
    pub spr: Sprite,
    moving: bool,
    direction: Direction,
    keyhold_count: u32
}

impl Player {
    pub fn new() -> Player{
        Self {
            spr: Sprite::new("reaper.png"),
            moving: false,
            // todo: make constant default direction, share with sprite
            direction: Direction::Down,
            keyhold_count: 0
        }
    }

    pub fn update(&mut self) {
        if self.moving {
            self.spr.movespr(self.direction);
        }
    }
}

impl Observer for Player {
    fn receive(&mut self, e: &Event) {
        // todo: this handling needs to be better
        // - make this less redundant
        // - currently if two keys are held down, and one key is released
        //   the player will continue to move in the same direction, regardless
        //   of which key was actually released - need to handle this more elegantly,
        //   likely need more granularity than just "halt"
        match e.name.as_str() {
            "up" => {
                self.direction = Direction::Up;
                self.moving = true;
                self.keyhold_count += 1;
            },
            "down" => {
                self.direction = Direction::Down;
                self.moving = true;
                self.keyhold_count += 1;
            },
            "left" => {
                self.direction = Direction::Left;
                self.moving = true;
                self.keyhold_count += 1;
            },
            "right" => {
                self.direction = Direction::Right;
                self.moving = true;
                self.keyhold_count += 1;
            },
            "halt" => {
                self.keyhold_count -= 1;
                if self.keyhold_count <= 0 {
                    self.moving = false;
                }
            }
            _ => ()
        }
    }
}