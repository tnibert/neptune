use std::collections::HashMap;
use std::cell::RefCell;


//#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Event <'a> {
    name: String,
    source: &'a Observable <'a>
}

impl <'a> Event <'a> {
    fn new(name: String, source: &'a Observable <'a>) -> Event <'a> {
        Self {
            name: name,
            source: source
        }
    }
}

trait EventReceiver {
    fn receive(&mut self, e: &Event);
}

pub struct Observer <'a> {
    receiver: &'a RefCell<dyn EventReceiver>
}

impl <'a> Observer <'a> {
    pub fn new(actioner: &'a RefCell<dyn EventReceiver>) -> Observer {
        Self {
            receiver: actioner
        }
    }

    fn receive(&self, e: &Event) {
        println!("Event received from {}: {}", e.source.name, e.name);
        let mut receiver = self.receiver.borrow_mut();
        receiver.receive(e);
    }
}

pub struct Observable <'a> {
    name: String,
    // idea: don't use an Observer object, map directly to a function/method/closure?
    subscribers: HashMap<String, Vec<&'a Observer <'a>>>
}

impl <'a> Observable <'a> {
    pub fn new(name: String) -> Observable <'a> {
        Self {
            name: name,
            subscribers: HashMap::new()
        }
    }

    // Subscribe an Observer to an event
    pub fn subscribe(&mut self, evt_name: String, subscriber: &'a Observer <'a>) {
        match self.subscribers.get_mut(&evt_name) {
            Some(vec) => vec.push(subscriber),
            None => {
                self.subscribers.insert(evt_name, vec![subscriber]);
            }
        };
    }

    // Remove an Observer from an event subscription
    fn unsubscribe(&mut self, evt_name: String, subscriber: &'a Observer <'a>) {
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
                    s.receive(&e);
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
        x: i32
    }

    impl EventReceiver for ObserverState {
        fn receive(&mut self, e: &Event) {
            //println!("received in event receiver");
            self.x = self.x + 1;
        }
    }

    // run with 'cargo test -- --nocapture' to see println! output
    // todo: make into proper unit tests
    #[test]
    fn test_observer_integration() {
        let mut obsable = Observable::new("my_observable".to_string());

        let mystate1 = RefCell::new(ObserverState{x: 0});
        let mystate2 = RefCell::new(ObserverState{x: 0});

        let obser1 = Observer::new(&mystate1);
        let obser2 = Observer::new(&mystate2);

        obsable.subscribe("test_event".to_string(), &obser1);
        obsable.subscribe("test_event".to_string(), &obser2);
        obsable.subscribe("bam".to_string(), &obser2);

        obsable.notify("test_event".to_string());
        obsable.notify("bam".to_string());

        obsable.unsubscribe("test_event".to_string(), &obser2);
        obsable.notify("test_event".to_string());

        println!("{}, {}", mystate1.borrow().x, mystate2.borrow().x);
    }
}

// notes
// mem::replace() can swap values of same type