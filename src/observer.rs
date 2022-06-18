use std::collections::HashMap;


//#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Event <'a>{
    name: String,
    source: &'a Observable<'a>
}

impl <'a> Event <'a> {
    fn new(name: String, source: &'a Observable<'a>) -> Event<'a> {
        Self {
            name: name,
            source: source
        }
    }
}

// todo: make trait
#[derive(PartialEq)]
pub struct Observer {
    //fn receive(&mut self, e: Event);
}

impl Observer {
    fn receive(&self, e: &Event) {
        println!("Event received from {}: {}", e.source.name, e.name);
    }
}

pub struct Observable <'a> {
    name: String,
    // idea: don't use an Observer object, map directly to a function/method/closure?
    subscribers: HashMap<String, Vec<&'a Observer>>
}

impl <'a> Observable <'a> {
    pub fn new(name: String) -> Observable <'a> {
        Self {
            name: name,
            subscribers: HashMap::new()
        }
    }

    // Subscribe an Observer to an event
    pub fn subscribe(&mut self, evt_name: String, subscriber: &'a Observer) {
        match self.subscribers.get_mut(&evt_name) {
            Some(vec) => vec.push(subscriber),
            None => {
                self.subscribers.insert(evt_name, vec![subscriber]);
            }
        };
    }

    // Remove an Observer from an event subscription
    fn unsubscribe(&mut self, evt_name: String, subscriber: &'a Observer) {
        match self.subscribers.get_mut(&evt_name) {
            Some(vec) => vec.retain(|x| !std::ptr::eq(*x, subscriber)),
            None => {}
        };
        ;
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

    // run with 'cargo test -- --nocapture' to see println! output
    #[test]
    fn test_observer_integration() {
        let mut obsable = Observable::new("my_observable".to_string());
        let obser1 = Observer{};
        let obser2 = Observer{};

        obsable.subscribe("test_event".to_string(), &obser1);
        obsable.subscribe("test_event".to_string(), &obser2);
        obsable.subscribe("bam".to_string(), &obser2);

        obsable.notify("test_event".to_string());
        obsable.notify("bam".to_string());

        obsable.unsubscribe("test_event".to_string(), &obser2);
        obsable.notify("test_event".to_string());
    }
}

// notes
// mem::replace() can swap values of same type