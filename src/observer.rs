use std::collections::HashMap;
use std::rc::Weak;
use std::rc::Rc;
use std::cell::RefCell;

// todo: should this rather be a struct?
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Event {
    Test1,
    // attach more info
    // todo: how to subscribe with the string being arbitrary?
    Test2(String)
}

trait Observer {
    // do we want to include a source?
    fn notify(&mut self, e: Event);
}

struct Observable {
    // hashmap containing key: Event, list of subscribed Observers
    // todo: don't use an Observer object, map directly to a function/method/closure
    subscriptions: HashMap<Event, Vec<Weak<RefCell<dyn Observer>>>>
}

impl Observable {
    fn new() -> Observable {
        Self {
            subscriptions: HashMap::new()
        }
    }

    // subscribe an Observer to an Event
    fn subscribe(&mut self, e: Event, o: Weak<RefCell<dyn Observer>>) {
        let subscriberlist = self.subscriptions.get_mut(&e);
        match subscriberlist {
            Some(val) => val.push(o),
            None => {
                self.subscriptions.insert(e, vec![o]);
            }
        }
    }

    // todo: remove an Observer from an Event subscription
    /*fn unsubscribe(&mut self, e: Event, o: Weak<RefCell<dyn Observer>>) {

    }*/
    
    // notify all subscribers to the given Event
    fn notify(&self, e: Event) {
        let subscriberlist = match self.subscriptions.get(&e) {
            Some(val) => val,
            // if no one is subscribed, return
            None => return
        };

        for weak_sub_ref in subscriberlist.iter() {
            // access through Weak
            let strong_sub_ref = weak_sub_ref.upgrade();
            match strong_sub_ref {
                Some(val) => val.borrow_mut().notify(e.clone()),
                // todo: the reference no longer exists, drop it
                None => ()
            }
        }
    }
}

// unit tests
#[cfg(test)]
mod tests {
    use super::*;

    struct TestObserver {}

    impl Observer for TestObserver {
        fn notify(&mut self, e: Event) {
            println!("Received {:?}", e);
        }
    }

    #[test]
    fn test_notify() {
        // create observers
        let testobserver1 = Rc::new(RefCell::new(TestObserver {}));
        let testdg1 = Rc::downgrade(&testobserver1);
        let testobserver2 = Rc::new(RefCell::new(TestObserver {}));
        let testdg2 = Rc::downgrade(&testobserver2);
        let testobserver3 = Rc::new(RefCell::new(TestObserver {}));
        let testdg3 = Rc::downgrade(&testobserver3);
        let testobserver4 = Rc::new(RefCell::new(TestObserver {}));
        let testdg4 = Rc::downgrade(&testobserver4);

        // create observable and subscriptions
        let mut testobservable: Observable = Observable::new();
        testobservable.subscribe(Event::Test1, testdg1);
        testobservable.subscribe(Event::Test1, testdg2);
        testobservable.subscribe(Event::Test1, testdg3);
        testobservable.subscribe(Event::Test1, testdg4);

        // notify events
        testobservable.notify(Event::Test1);
        // todo: make this one work with a variable string
        testobservable.notify(Event::Test2("test".to_string()));
    }
}
