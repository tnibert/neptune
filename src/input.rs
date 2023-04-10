use crate::observer::{Observable, Listener};
use piston::input::*;
use std::collections::BTreeSet;
use std::rc::Rc;

pub struct Input {
    signals_out: Observable,
    keys_down: BTreeSet<piston_window::Key>
}

impl Input {
    pub fn new() -> Input {
        Self {
            signals_out: Observable::new("input".to_string()),
            keys_down: BTreeSet::new()
        }
    }

    pub fn subscribe(&mut self, subscriber: Rc<Listener>, event_names: Vec<&str>) {
        for en in event_names {
            self.signals_out.subscribe(en.to_string(), subscriber.clone());
        }
    }

    // todo: decouple input from piston
    pub fn handle_event(&mut self, e: &Event) {
        // key pressed down
        if let Some(Button::Keyboard(k)) = e.press_args() {
            self.keys_down.insert(k);
        }

        // key released
        if let Some(Button::Keyboard(k)) = e.release_args() {
            self.keys_down.remove(&k);
        }

        for k in &self.keys_down {
            match k {
                piston_window::Key::Right => {
                    self.signals_out.notify("right".to_string());
                },
                piston_window::Key::Left => {
                    self.signals_out.notify("left".to_string());
                },
                piston_window::Key::Down => {
                    self.signals_out.notify("down".to_string());
                },
                piston_window::Key::Up => {
                    self.signals_out.notify("up".to_string());
                }
                _ => {},
            }
        }
        
    }
}