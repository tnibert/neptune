use crate::observer::Observable;
use crate::sprite::Sprite;
use crate::sprite::Direction;
use crate::observer::{Listener, NeptuneEvent};
use crate::gameobject::GameObject;
use crate::collision::{Rect, new_point};
use crate::game::{SCREEN_WIDTH, SCREEN_HEIGHT};

use std::rc::Rc;

pub const PLAYER_SPEED: i64 = 3;

// represents the vantage point
pub struct Player {
    spr: Sprite,
    pub observer: Rc<Listener>,
    pub signals_out: Observable,
    bounds: Rect
}

impl Player {
    pub fn new(bounds: Rect) -> Player {
        Self {
            spr: Sprite::new("reaper.png", PLAYER_SPEED, new_point((SCREEN_WIDTH / 2) as i64,(SCREEN_HEIGHT / 2) as i64), Direction::Down),
            observer: Rc::new(Listener::new()),
            signals_out: Observable::new("player".to_string()),
            bounds: bounds
        }
    }

    pub fn subscribe(&mut self, subscriber: Rc<Listener>, events: Vec<NeptuneEvent>) {
        for en in events {
            self.signals_out.subscribe(en, subscriber.clone());
        }
    }

    fn should_move_world_horizontal(&self) -> bool {
        let limit = (SCREEN_WIDTH as i64)/2;
        if let Some(pos) = self.spr.position() {
            pos.x > limit && pos.x < self.bounds.w - limit
        } else {
            false
        }
    }

    fn should_move_world_vertical(&self) -> bool {
        let limit = (SCREEN_HEIGHT as i64)/2;
        if let Some(pos) = self.spr.position() {
            pos.y > limit && pos.y < self.bounds.h - limit
        } else {
            false
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
                    self.spr.movespr(Direction::Up);
                    if self.should_move_world_vertical() {
                        self.signals_out.notify(NeptuneEvent::Up);
                    }
                },
                NeptuneEvent::Down => {
                    self.spr.set_facing(Direction::Down);
                    self.spr.movespr(Direction::Down);
                    if self.should_move_world_vertical() {
                        self.signals_out.notify(NeptuneEvent::Down);
                    }
                },
                NeptuneEvent::Left => {
                    self.spr.set_facing(Direction::Left);
                    self.spr.movespr(Direction::Left);
                    if self.should_move_world_horizontal() {
                        self.signals_out.notify(NeptuneEvent::Left);
                    }
                },
                NeptuneEvent::Right => {
                    self.spr.set_facing(Direction::Right);
                    self.spr.movespr(Direction::Right);
                    if self.should_move_world_horizontal() {
                        self.signals_out.notify(NeptuneEvent::Right);
                    }
                },
                _ => ()
            }
        }

        self.spr.update();
    }
}