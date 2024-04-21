use crate::sprite::Sprite;
use crate::sprite::Direction;
use crate::observer::{Listener, NeptuneEvent};
use crate::gameobject::GameObject;
use crate::collision::{Rect, new_point};
use crate::game::{SCREEN_WIDTH, SCREEN_HEIGHT};

use std::rc::Rc;

const PLAYER_SPEED: i64 = 1;

// handles player, receives input signals
pub struct Player {
    spr: Sprite,
    pub observer: Rc<Listener>
}

impl Player {
    pub fn new() -> Player {
        Self {
            spr: Sprite::new("reaper.png", PLAYER_SPEED, new_point((SCREEN_WIDTH / 2) as i64,(SCREEN_HEIGHT / 2) as i64), Direction::Down),
            observer: Rc::new(Listener::new())
        }
    }
}

impl GameObject for Player {
    fn render(&self) -> Option<im::RgbaImage> {
        return self.spr.render();
    }

    fn position(&self) -> Option<Rect> {
        return self.spr.position();
    }

    fn update(&mut self) {
        for e in self.observer.poll_evt() {
            match e {
                NeptuneEvent::Up => {
                    self.spr.set_facing(Direction::Up);
                },
                NeptuneEvent::Down => {
                    self.spr.set_facing(Direction::Down);
                },
                NeptuneEvent::Left => {
                    self.spr.set_facing(Direction::Left);
                },
                NeptuneEvent::Right => {
                    self.spr.set_facing(Direction::Right);
                },
                _ => ()
            }
        }

        self.spr.update();
    }
}