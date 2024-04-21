use crate::imgload::load_image_asset_buffer;
use crate::observer::{Listener, NeptuneEvent, Observable};
use crate::gameobject::GameObject;
use crate::collision::Rect;
use crate::player::PLAYER_SPEED;
use crate::game::{SCREEN_WIDTH, SCREEN_HEIGHT};

use std::rc::Rc;
//use std::time::Instant;

const STEP: i64 = PLAYER_SPEED;

pub struct Background {
    full_image: im::RgbaImage,
    pub observer: Rc<Listener>,
    pub signals_out: Observable,
    crop_corner_x: i64,
    crop_corner_y: i64
}

impl Background {
    pub fn new(fname: &str) -> Self {
        Self {
            full_image: load_image_asset_buffer(fname),
            observer: Rc::new(Listener::new()),
            signals_out: Observable::new("background".to_string()),
            crop_corner_x: 0,
            crop_corner_y: 0
        }
    }

    // return a rect of the current window in world coordinates
    pub fn window(&self) -> Rect {
        Rect {
            x: self.crop_corner_x,
            y: self.crop_corner_y,
            w: SCREEN_WIDTH as i64,
            h: SCREEN_HEIGHT as i64
        }
    }
}

impl GameObject for Background {

    /* 
     * clip the background to fit current window based
     * on position
     */
    fn render(&self) -> Option<im::RgbaImage> {
        //let start = Instant::now();
        let cropped = im::imageops::crop_imm(&self.full_image,
                      self.crop_corner_x as u32,
                      self.crop_corner_y as u32,
                      SCREEN_WIDTH as u32,
                      SCREEN_HEIGHT as u32).to_image();
        //let duration = start.elapsed();
        //println!("bg render: {:?}", duration);
        return Some(cropped);
    }

    /*
     * We always position at (0,0) to draw over whole screen
     */
    fn position(&self) -> Option<Rect> {
        return Some(Rect {
            x: 0,
            y: 0,
            w: SCREEN_WIDTH as i64,
            h: SCREEN_HEIGHT as i64
        });
    }

    // todo: if the player reaches end of screen, player should leave the center
    // and be able to walk to edge of map
    fn update(&mut self) {
        for e in self.observer.poll_evt() {
            match e {
                NeptuneEvent::Up => {
                    self.crop_corner_y -= STEP;
                    self.signals_out.notify(NeptuneEvent::VisibilityChange(self.window()))
                },
                NeptuneEvent::Down => {
                    self.crop_corner_y += STEP;
                    self.signals_out.notify(NeptuneEvent::VisibilityChange(self.window()))
                },
                NeptuneEvent::Left => {
                    self.crop_corner_x -= STEP;
                    self.signals_out.notify(NeptuneEvent::VisibilityChange(self.window()))
                },
                NeptuneEvent::Right => {
                    self.crop_corner_x += STEP;
                    self.signals_out.notify(NeptuneEvent::VisibilityChange(self.window()))
                },
                _ => ()
            }
        }
    }
}