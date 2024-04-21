use crate::observer::{Observable, Listener, NeptuneEvent};
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

    pub fn subscribe(&mut self, subscriber: Rc<Listener>, events: Vec<NeptuneEvent>) {
        for en in events {
            self.signals_out.subscribe(en, subscriber.clone());
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
                    self.signals_out.notify(NeptuneEvent::Right);
                },
                piston_window::Key::Left => {
                    self.signals_out.notify(NeptuneEvent::Left);
                },
                piston_window::Key::Down => {
                    self.signals_out.notify(NeptuneEvent::Down);
                },
                piston_window::Key::Up => {
                    self.signals_out.notify(NeptuneEvent::Up);
                }
                _ => {},
            }
        }
        
    }
}