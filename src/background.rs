use crate::imgload::load_image_asset_buffer;
use crate::observer::{Listener, NeptuneEvent, Observable};
use crate::gameobject::GameObject;
use crate::collision::Rect;
use crate::player::PLAYER_SPEED;
use crate::game::{SCREEN_WIDTH, SCREEN_HEIGHT};
use crate::viewport::Viewport;

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

    // return a rect of the current viewport in world coordinates
    fn viewport(&self) -> Viewport {
        Viewport::new(Rect {
            x: self.crop_corner_x,
            y: self.crop_corner_y,
            w: SCREEN_WIDTH as i64,
            h: SCREEN_HEIGHT as i64
        })
    }

    pub fn width(&self) -> u32 {
        return self.full_image.width()
    }

    pub fn height(&self) -> u32 {
        return self.full_image.height()
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
     * todo: This is janky, those width and height attributes are not correct
     * but those values allow the program to function correctly.  Need to revisit.
     */
    fn position(&self) -> Option<Rect> {
        return Some(Rect {
            x: self.crop_corner_x as i64,
            y: self.crop_corner_y as i64,
            w: self.crop_corner_x+SCREEN_WIDTH as i64,
            h: self.crop_corner_y+SCREEN_HEIGHT as i64
        });
    }

    // todo: if the player reaches end of screen, player should leave the center
    // and be able to walk to edge of map
    fn update(&mut self) {
        for e in self.observer.poll_evt() {
            match e {
                NeptuneEvent::Up => {
                    self.crop_corner_y -= STEP;
                    self.signals_out.notify(NeptuneEvent::VisibilityChange(self.viewport()))
                },
                NeptuneEvent::Down => {
                    self.crop_corner_y += STEP;
                    self.signals_out.notify(NeptuneEvent::VisibilityChange(self.viewport()))
                },
                NeptuneEvent::Left => {
                    self.crop_corner_x -= STEP;
                    self.signals_out.notify(NeptuneEvent::VisibilityChange(self.viewport()))
                },
                NeptuneEvent::Right => {
                    self.crop_corner_x += STEP;
                    self.signals_out.notify(NeptuneEvent::VisibilityChange(self.viewport()))
                },
                _ => ()
            }
        }
    }
}