use crate::sprite::Sprite;
use crate::observer::Observer;
use crate::observer::Event;

use std::cell::RefCell;

const PLAYER_W: u32 = 26;
const PLAYER_H: u32 = 36;
const PLAYER_SPEED: i32 = 5;

// handles player, received input
//#[derive(Debug)]
pub struct Player {
    // todo: make not pub
    pub spr: Sprite
}

impl Player {
    pub fn new() -> Player{
        Self {
            spr: Sprite::new(PLAYER_W, PLAYER_H, PLAYER_SPEED)
        }
    }
}

impl Observer for Player {
    fn receive(&mut self, e: &Event) {
        match e.name.as_str() {
            "up" => self.spr.movespr(0, -self.spr.speed),
            "down" => self.spr.movespr(0, self.spr.speed),
            "left" => self.spr.movespr(-self.spr.speed, 0),
            "right" => self.spr.movespr(self.spr.speed, 0),
            _ => ()
        }
    }
}