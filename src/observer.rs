use std::collections::HashMap;
use core::cell::RefCell;
use std::rc::Rc;

/*
 * Modified implementation of observer pattern for propogating events
 */

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Event {
    pub name: String,
}

pub struct Listener {
    ledger: RefCell<Vec<Event>>
}

impl Listener {
    pub fn new() -> Listener {
        Self {
            ledger: RefCell::new(Vec::new())
        }
    }

    fn receive(&self, e: &Event) {
        // appends to end of ledger
        self.ledger.borrow_mut().push(e.clone());
    }

    pub fn poll_evt(&self) -> Vec<Event> {
        let ret = self.ledger.borrow().clone();
        self.ledger.borrow_mut().clear();
        return ret;
    }
}

pub struct Observable {
    name: String,
    // idea: don't use a Listener object, map directly to a function/method/closure?
    subscribers: HashMap<String, Vec<Rc<Listener>>>
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
    pub fn subscribe(&mut self, evt_name: String, subscriber: Rc<Listener>) {
        match self.subscribers.get_mut(&evt_name) {
            Some(vec) => vec.push(subscriber),
            None => {
                self.subscribers.insert(evt_name, vec![subscriber]);
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
    pub fn notify(&self, evt_name: String) {
        let e = Rc::new(Event{name: evt_name.clone()});
        match self.subscribers.get(&evt_name) {
            Some(to_notify) => {
                // immutable iteration
                for s in to_notify {
                    s.receive(&e);
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

        obsable.subscribe("test".to_string(), l1.clone());
        obsable.subscribe("test".to_string(), l2.clone());

        obsable.notify("test".to_string());

        let e1 = l1.poll_evt();
        let e2 = l2.poll_evt();

        assert_eq!(e1[0].name, "test");
        assert_eq!(e2[0].name, "test");
    }
}
