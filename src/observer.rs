use std::collections::HashMap;
use std::cell::RefCell;

//#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Event <'a> {
    pub name: String,
    pub source: &'a Observable <'a>
}

impl <'a> Event <'a> {
    /*fn new(name: String, source: &'a Observable <'a>) -> Event <'a> {
        Self {
            name: name,
            source: source
        }
    }*/
}

pub trait Observer {
    fn receive(&mut self, e: &Event);
}

// is it reasonable to make this a trait? or should a struct just hold an Observable?
pub struct Observable <'a> {
    name: String,
    // idea: don't use an Observer object, map directly to a function/method/closure?
    subscribers: HashMap<String, Vec<&'a RefCell<dyn Observer>>>
}

impl <'a> Observable <'a> {
    pub fn new(name: String) -> Observable <'a> {
        Self {
            name: name,
            subscribers: HashMap::new()
        }
    }

    // Subscribe an Observer to an event
    // idea: should explicitly prevent subscribing the same Observer twice?
    pub fn subscribe(&mut self, evt_name: String, subscriber: &'a RefCell<dyn Observer>) {
        match self.subscribers.get_mut(&evt_name) {
            Some(vec) => vec.push(subscriber),
            None => {
                self.subscribers.insert(evt_name, vec![subscriber]);
            }
        };
    }

    // Remove an Observer from an event subscription
    fn unsubscribe(&mut self, evt_name: String, subscriber: &'a RefCell<dyn Observer>) {
        match self.subscribers.get_mut(&evt_name) {
            Some(vec) => vec.retain(|x| !std::ptr::eq(*x, subscriber)),
            None => {}
        };
    }
    
    // Notify all subscribers to the given Event
    pub fn notify(&self, evt_name: String) {
        let e = Event{name: evt_name.clone(),
                      source: self};
        match self.subscribers.get(&evt_name) {
            Some(to_notify) => {
                // immutable iteration
                for s in to_notify {
                    let mut receiver = s.borrow_mut();
                    receiver.receive(&e);
                }
            },
            None => {}
        }
    }
}

// unit tests
#[cfg(test)]
mod tests {
    use super::*;

    struct ObserverState {
        counter: i32
    }

    impl Observer for ObserverState {
        fn receive(&mut self, e: &Event) {
            println!("Event received from {}: {}", e.source.name, e.name);
            self.counter = self.counter + 1;
        }
    }

    #[test]
    fn test_subscribe() {
        let mut obsable = Observable::new("my_observable".to_string());
        let mystate1 = RefCell::new(ObserverState{counter: 0});

        obsable.notify("test_event".to_string());
        assert_eq!(mystate1.borrow().counter, 0);
        obsable.subscribe("test_event".to_string(), &mystate1);
        obsable.notify("test_event".to_string());
        assert_eq!(mystate1.borrow().counter, 1);
        obsable.notify("test_event".to_string());
        assert_eq!(mystate1.borrow().counter, 2);
    }

    #[test]
    fn test_unsubscribe() {
        let mut obsable = Observable::new("my_observable".to_string());
        let mystate1 = RefCell::new(ObserverState{counter: 0});

        obsable.subscribe("test_event".to_string(), &mystate1);
        obsable.notify("test_event".to_string());
        assert_eq!(mystate1.borrow().counter, 1);
        obsable.unsubscribe("test_event".to_string(), &mystate1);
        obsable.notify("test_event".to_string());
        assert_eq!(mystate1.borrow().counter, 1);
    }


    // run with 'cargo test -- --nocapture' to see println! output
    #[test]
    fn test_observer_integration() {
        let mut obsable = Observable::new("my_observable".to_string());

        let mystate1 = RefCell::new(ObserverState{counter: 0});
        let mystate2 = RefCell::new(ObserverState{counter: 0});

        obsable.subscribe("test_event".to_string(), &mystate1);
        obsable.subscribe("test_event".to_string(), &mystate2);
        obsable.subscribe("bam".to_string(), &mystate2);

        obsable.notify("test_event".to_string());
        obsable.notify("bam".to_string());

        obsable.unsubscribe("test_event".to_string(), &mystate2);
        obsable.notify("test_event".to_string());

        println!("{}, {}", mystate1.borrow().counter, mystate2.borrow().counter);
    }
}

// notes
// mem::replace() can swap values of same type