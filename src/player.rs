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
    direction: Direction
}

impl Player {
    pub fn new() -> Player{
        Self {
            spr: Sprite::new("reaper.png"),
            moving: false,
            // todo: make constant default direction, share with sprite
            direction: Direction::Down
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
        match e.name.as_str() {
            "up" => {
                self.direction = Direction::Up;
                self.moving = true;
            },
            "down" => {
                self.direction = Direction::Down;
                self.moving = true;
            },
            "left" => {
                self.direction = Direction::Left;
                self.moving = true;
            },
            "right" => {
                self.direction = Direction::Right;
                self.moving = true;
            },
            "halt" => self.moving = false,
            _ => ()
        }
    }
}