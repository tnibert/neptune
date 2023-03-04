use crate::graphics::load_image_asset_buffer;
use crate::observer::Listener;
use crate::gameobject::GameObject;
use crate::collision::Rect;
use crate::game::{SCREEN_WIDTH, SCREEN_HEIGHT};

use std::rc::Rc;

pub struct Background {
    full_image: im::RgbaImage,
    //pub observer: Rc<Listener>
}

impl Background {
    pub fn new(fname: &str) -> Self {
        Self {
            full_image: load_image_asset_buffer(fname)
        }
    }
}

impl GameObject for Background {

    /* clip the background to fit current window based
     * on position
     */
    fn render(&self) -> Option<im::RgbaImage> {
        let cropped = im::imageops::crop_imm(&self.full_image, 0,
                      0,
                      SCREEN_WIDTH as u32,
                      SCREEN_HEIGHT as u32).to_image();
        return Some(cropped);
    }

    fn position(&self) -> Option<Rect> {
        return Some(Rect {
            x: 0.0,
            y: 0.0,
            w: SCREEN_WIDTH as f64,
            h: SCREEN_HEIGHT as f64
        });
    }

    fn update(&mut self) {
        /*for e in self.observer.poll_evt() {
            match e.as_str() {
                "up" => {
                    //self.spr.movespr(Direction::Up);
                },
                "down" => {
                    //self.spr.movespr(Direction::Down);
                },
                "left" => {
                    //self.spr.movespr(Direction::Left);
                },
                "right" => {
                    //self.spr.movespr(Direction::Right);
                },
                _ => ()
            }
        }*/
    }
}