use std::rc::Rc;

use crate::observer::NeptuneEvent;
use crate::sprite::Direction;
use crate::sprite::Sprite;
use crate::observer::Listener;
use crate::gameobject::GameObject;
use crate::game::{SCREEN_WIDTH, SCREEN_HEIGHT};
use crate::collision;

pub struct NPC {
    spr: Sprite,
    pub observer: Rc<Listener>
}

impl NPC {
    pub fn new(initial_direction: Direction) -> NPC {
        Self {
            spr: Sprite::new("reaper.png", 
                       0,
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
                _ => ()
            }
        }

        self.spr.update();
    }
}