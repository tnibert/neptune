use crate::sprite::Sprite;
use crate::sprite::Direction;
use crate::observer::Observer;
use crate::observer::Event;
use crate::renderable::Render;
use crate::updateable::Update;

const PLAYER_SPEED: f64 = 1.0;

// handles player, receives input signals
pub struct Player {
    spr: Sprite
}

impl Player {
    pub fn new() -> Player{
        Self {
            spr: Sprite::new("reaper.png", PLAYER_SPEED)
        }
    }
}

impl Update for Player {
    fn update(&mut self) {
        self.spr.update();
    }
}

impl Render for Player {
    fn render(&self) -> &im::RgbaImage {
        return self.spr.render();
    }

    fn position(&self) -> (f64, f64) {
        return self.spr.position();
    }
}

impl Observer for Player {
    fn receive(&mut self, e: &Event) {
        match e.name.as_str() {
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
}