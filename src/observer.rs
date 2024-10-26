use std::collections::HashMap;
use core::cell::RefCell;
use std::rc::Rc;
use std::mem::{self, Discriminant};

use crate::viewport::Viewport;

/*
 * Modified implementation of observer pattern for propogating events
 */

 // todo: need to implement an Eq such that all VisibilityChange are equal
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum NeptuneEvent {
    Left,
    Right,
    Down,
    Up,
    Talk,
    VisibilityChange(Viewport)
}

pub struct Listener {
    ledger: RefCell<Vec<NeptuneEvent>>
}

impl Listener {
    pub fn new() -> Listener {
        Self {
            ledger: RefCell::new(Vec::new())
        }
    }

    fn receive(&self, e: &NeptuneEvent) {
        // appends to end of ledger
        self.ledger.borrow_mut().push(e.clone());
    }

    pub fn poll_evt(&self) -> Vec<NeptuneEvent> {
        let ret = self.ledger.borrow().clone();
        self.ledger.borrow_mut().clear();
        return ret;
    }
}

pub struct Observable {
    name: String,
    // Discriminant allows us to ignore the content of the NeptuneEvent for purposes of HashMap key
    subscribers: HashMap<Discriminant<NeptuneEvent>, Vec<Rc<Listener>>>
}

impl Observable {
    pub fn new(name: String) -> Observable {
        Self {
            name: name,
            subscribers: HashMap::new()
        }
    }

    // Subscribe an Observer to an event
    // idea: should explicitly prevent subscribing the same Observer twice?
    pub fn subscribe(&mut self, evt: NeptuneEvent, subscriber: Rc<Listener>) {
        match self.subscribers.get_mut(&mem::discriminant(&evt)) {
            Some(vec) => vec.push(subscriber),
            None => {
                self.subscribers.insert(mem::discriminant(&evt), vec![subscriber]);
            }
        };
    }

    // Remove an Observer from an event subscription
    /*fn unsubscribe(&mut self, evt_name: String, subscriber: &'a RefCell<dyn Observer>) {
        match self.subscribers.get_mut(&evt_name) {
            Some(vec) => vec.retain(|x| !std::ptr::eq(*x, subscriber)),
            None => {}
        };
    }*/

    // Notify all subscribers to the given Event
    pub fn notify(&self, evt: NeptuneEvent) {
        match self.subscribers.get(&mem::discriminant(&evt)) {
            Some(to_notify) => {
                // immutable iteration
                for s in to_notify {
                    s.receive(&evt);
                }
            },
            None => {}
        }
    }
}

// unit tests
// run with 'cargo test -- --nocapture' to see println! output
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integration() {
        let mut obsable = Observable::new("my_observable".to_string());
        let l1 = Rc::new(Listener::new());
        let l2 = Rc::new(Listener::new());

        obsable.subscribe(NeptuneEvent::Up, l1.clone());
        obsable.subscribe(NeptuneEvent::Up, l2.clone());

        obsable.notify(NeptuneEvent::Up);

        let e1 = l1.poll_evt();
        let e2 = l2.poll_evt();

        assert_eq!(e1[0], NeptuneEvent::Up);
        assert_eq!(e2[0], NeptuneEvent::Up);
    }
}
