use std::rc::Rc;

use crate::observer::NeptuneEvent;
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
                       STEP as i64,
            collision::new_point((SCREEN_WIDTH / 3) as i64, (SCREEN_HEIGHT / 3) as i64),
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
            match e {
                NeptuneEvent::Talk => {

                },
                // todo: don't subscribe to all of these, have something emit a single signal with movement diff and direction attached
                NeptuneEvent::Up => {
                    self.spr.movespr(Direction::Down);
                },
                NeptuneEvent::Down => {
                    self.spr.movespr(Direction::Up);
                },
                NeptuneEvent::Left => {
                    self.spr.movespr(Direction::Right);
                },
                NeptuneEvent::Right => {
                    self.spr.movespr(Direction::Left);
                },
                _ => ()
            }
        }

        self.spr.update();
    }
}