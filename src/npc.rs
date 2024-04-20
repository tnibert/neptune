use std::rc::Rc;

use crate::sprite::Direction;
use crate::sprite::Sprite;
use crate::observer::Listener;
use crate::gameobject::GameObject;
use crate::game::{SCREEN_WIDTH, SCREEN_HEIGHT};
use crate::collision;
use crate::background::STEP;

pub struct NPC {
    spr: Sprite,
    pub observer: Rc<Listener>
}

impl NPC {
    pub fn new(initial_direction: Direction) -> NPC {
        Self {
            spr: Sprite::new("reaper.png", 
                       STEP as f64,
            collision::new_point((SCREEN_WIDTH / 3) as f64, (SCREEN_HEIGHT / 3) as f64), 
                              initial_direction),
            observer: Rc::new(Listener::new())
        }
    }
}

impl GameObject for NPC {
    fn render(&self) -> Option<im::RgbaImage> {
        return self.spr.render();
    }

    fn position(&self) -> Option<collision::Rect> {
        return self.spr.position();
    }

    fn update(&mut self) {
        for e in self.observer.poll_evt() {
            match e.as_str() {
                "talk" => {

                },
                // todo: don't subscribe to all of these, have something emit a single signal with movement diff and direction attached
                "up" => {
                    self.spr.movespr(Direction::Down);
                },
                "down" => {
                    self.spr.movespr(Direction::Up);
                },
                "left" => {
                    self.spr.movespr(Direction::Right);
                },
                "right" => {
                    self.spr.movespr(Direction::Left);
                },
                _ => ()
            }
        }

        self.spr.update();
    }
}