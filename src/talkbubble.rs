use std::rc::Rc;
use std::include_bytes;

use imageproc::drawing::draw_text_mut;
use ab_glyph::{FontRef, PxScale};

use crate::collision::Rect;
use crate::observer::NeptuneEvent;
use crate::observer::Listener;
use crate::gameobject::GameObject;
use crate::collision;

// todo: make parameters from magic values

// todo: portability
const FONT: &[u8] = include_bytes!("/usr/share/fonts/noto/NotoSerif-Regular.ttf");

pub struct TalkBubble {
    bounding_rect: Rect,
    text: String,
    pub observer: Rc<Listener>
}

impl TalkBubble {
    pub fn new() -> TalkBubble {
        Self {
            bounding_rect: Rect{x: 100, y: 100, w: 120, h: 100},
            text: "Hi Willow!".to_string(),
            observer: Rc::new(Listener::new())
        }
    }
}

impl GameObject for TalkBubble {
    fn render(&self) -> Option<im::RgbaImage> {
        // Load font
        let font = FontRef::try_from_slice(FONT).unwrap();

        let intended_text_height = 12.2;
        let scale = PxScale {
            x: intended_text_height * 2.0,
            y: intended_text_height,
        };

        // apply text to the image
        let mut image = imageproc::image::RgbaImage::new(self.bounding_rect.w as u32, self.bounding_rect.h as u32);
        draw_text_mut(&mut image, imageproc::image::Rgba([0u8, 0u8, 0u8, 255u8]), 10, 10, scale, &font, &self.text.to_string());

        // convert imageproc::image to im::image
        let new_img = im::RgbaImage::from_raw(image.width(), image.height(), image.into_raw()).unwrap();
        return Some(new_img);
    }

    fn position(&self) -> Option<collision::Rect> {
        return Some(self.bounding_rect.clone());
    }

    fn update(&mut self) {
        for e in self.observer.poll_evt() {
            match e {
                NeptuneEvent::Talk => {

                },
                _ => ()
            }
        }
    }
}