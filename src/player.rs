use crate::sprite::Sprite;
use crate::sprite::Direction;
use crate::observer::Listener;
use crate::gameobject::GameObject;

use std::rc::Rc;

const PLAYER_SPEED: f64 = 1.0;

// handles player, receives input signals
pub struct Player {
    spr: Sprite,
    pub observer: Rc<Listener>
}

impl Player {
    pub fn new() -> Player{
        Self {
            spr: Sprite::new("reaper.png", PLAYER_SPEED),
            observer: Rc::new(Listener::new())
        }
    }
}

impl GameObject for Player {
    fn render(&self) -> Option<im::RgbaImage> {
        return self.spr.render();
    }

    fn position(&self) -> Option<(f64, f64)> {
        return self.spr.position();
    }

    fn update(&mut self) {
        for e in self.observer.poll_evt() {
            match e.as_str() {
                "up" => {
                    self.spr.movespr(Direction::Up);
                },
                "down" => {
                    self.spr.movespr(Direction::Down);
                },
                "left" => {
                    self.spr.movespr(Direction::Left);
                },
                "right" => {
                    self.spr.movespr(Direction::Right);
                },
                _ => ()
            }
        }

        self.spr.update();
    }
}