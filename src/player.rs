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
    pub spr: Sprite
}

impl Player {
    pub fn new() -> Player{
        Self {
            spr: Sprite::new("reaper.png")
        }
    }
}

impl Observer for Player {
    fn receive(&mut self, e: &Event) {
        match e.name.as_str() {
            "up" => self.spr.movespr(Direction::Up),
            "down" => self.spr.movespr(Direction::Down),
            "left" => self.spr.movespr(Direction::Left),
            "right" => self.spr.movespr(Direction::Right),
            _ => ()
        }
    }
}