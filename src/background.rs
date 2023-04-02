use crate::imgload::load_image_asset_buffer;
use crate::observer::Listener;
use crate::gameobject::GameObject;
use crate::collision::Rect;
use crate::game::{SCREEN_WIDTH, SCREEN_HEIGHT};

use std::rc::Rc;
use std::time::Instant;

const STEP: u32 = 3;

pub struct Background {
    full_image: im::RgbaImage,
    pub observer: Rc<Listener>,
    crop_corner_x: u32,
    crop_corner_y: u32
}

impl Background {
    pub fn new(fname: &str) -> Self {
        Self {
            full_image: load_image_asset_buffer(fname),
            observer: Rc::new(Listener::new()),
            crop_corner_x: 0,
            crop_corner_y: 0
        }
    }
}

impl GameObject for Background {

    /* 
     * clip the background to fit current window based
     * on position
     */
    fn render(&self) -> Option<im::RgbaImage> {
        let start = Instant::now();
        let cropped = im::imageops::crop_imm(&self.full_image, self.crop_corner_x,
                      self.crop_corner_y,
                      SCREEN_WIDTH as u32,
                      SCREEN_HEIGHT as u32).to_image();
        let duration = start.elapsed();
        println!("bg render: {:?}", duration);
        return Some(cropped);
    }

    /*
     * We always position at (0,0) to draw over whole screen
     */
    fn position(&self) -> Option<Rect> {
        return Some(Rect {
            x: 0.0,
            y: 0.0,
            w: SCREEN_WIDTH as f64,
            h: SCREEN_HEIGHT as f64
        });
    }

    // todo: if the player reaches end of screen, player should leave the center
    // and be able to walk to edge of map
    fn update(&mut self) {
        for e in self.observer.poll_evt() {
            match e.as_str() {
                "up" => {
                    if self.crop_corner_y >= STEP {
                        self.crop_corner_y -= STEP;
                    }
                },
                "down" => {
                    if self.crop_corner_y <= self.full_image.height() - STEP {
                        self.crop_corner_y += STEP;
                    }
                },
                "left" => {
                    if self.crop_corner_x >= STEP {
                        self.crop_corner_x -= STEP;
                    }
                },
                "right" => {
                    if self.crop_corner_x <= self.full_image.width() - STEP {
                        self.crop_corner_x += STEP;
                    }
                },
                _ => ()
            }
        }
    }
}