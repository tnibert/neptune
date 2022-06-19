use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use crate::observer::Observable;

pub struct Input <'a> {
    event_pump: EventPump,
    pub observable: Observable <'a>
}

impl <'a> Input <'a> {
    pub fn new(evt_recv: EventPump) -> Input <'a> {
        Self {
            event_pump: evt_recv,
            observable: Observable::new("input".to_string())
        }
    }

    pub fn poll_input(&mut self) -> bool {
        // event pump is queried to find out if there are any pending events
        for event in self.event_pump.poll_iter() {
            println!("{:?}", event);
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    // todo: subscribe Game
                    //&self.observable.notify("exit".to_string());
                    return true;
                },
                // player control
                // todo: allow diagonal movement
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    &self.observable.notify("left".to_string());
                },
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    &self.observable.notify("right".to_string());
                },
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    &self.observable.notify("up".to_string());
                },
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    &self.observable.notify("down".to_string());
                },
                _ => {}
            }
        }
        return false;
    }
}