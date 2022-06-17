use std::collections::HashMap;
use std::rc::Weak;
use std::rc::Rc;
use std::cell::RefCell;

// todo: should this rather be a struct?
/*#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Event {
    KEY_UP,
    KEY_DOWN,
    KEY_LEFT,
    KEY_RIGHT,
    Test1,
    // todo: how to subscribe with the string being arbitrary?
    Test2(String)
}*/


#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Event {
    name: String
    // source: Observable
}

impl Event {
    fn new(name: String) -> Event {
        Self {
            name: name
        }
    }
}

// todo: make trait
pub struct Observer {
    //fn receive(&mut self, e: Event);
}

impl Observer {
    fn receive(&self, e: Event) {
        println!("Event received: {}", e.name);
    }
}

pub struct Observable <'a> {
    // hashmap containing key: Event, list of subscribed Observers
    // todo: don't use an Observer object, map directly to a function/method/closure
    //subscriptions: HashMap<Event, Vec<Weak<RefCell<T>>>>
    subscribers: Vec<&'a Observer>
}

/*impl <T> Observable<T> 
where 
    T: Observer,*/
impl <'a> Observable <'a> {
    pub fn new() -> Observable <'a> {
        Self {
            subscribers: Vec::new()  //HashMap::new()
        }
    }

    // subscribe an Observer to an Event
    /*pub fn subscribe(&mut self, e: Event, o: &Rc<RefCell<T>>) {
        let obs_weak = Rc::downgrade(o);

        let subscriberlist = self.subscriptions.get_mut(&e);
        match subscriberlist {
            Some(val) => val.push(obs_weak),
            None => {
                self.subscriptions.insert(e, vec![obs_weak]);
            }
        }
    }*/
    pub fn subscribe(&mut self, subscriber: &'a Observer) {
        self.subscribers.push(subscriber);
    }

    // todo: remove an Observer from an Event subscription
    /*fn unsubscribe(&mut self, e: Event, o: Weak<RefCell<dyn Observer>>) {

    }*/
    
    // notify all subscribers to the given Event

    /*pub fn notify(&self, e: Event) {
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
    }*/
    pub fn notify(&self, e: Event) {
        // immutable iteration
        for s in &self.subscribers {
            s.receive(e.clone());
        }
    }
}

// unit tests
#[cfg(test)]
mod tests {
    use super::*;

    /*struct TestObserver {
        t1_notifies: u32,
        t2_notifies: u32
    }

    impl TestObserver {
        fn new() -> TestObserver {
            Self {
                t1_notifies: 0,
                t2_notifies: 0
            }
        }
    }

    impl Observer for TestObserver {
        fn notify(&mut self, e: Event) {
            println!("Received {:?}", e);
            match e {
                Event::Test1 => self.t1_notifies = self.t1_notifies + 1,
                Event::Test2(mystr) => self.t2_notifies = self.t2_notifies + 1,
                _ => ()
            }
        }
    }1*/

    /*#[test]
    fn test_notify() {
        // create observers
        let testobserver1 = Rc::new(RefCell::new(TestObserver::new()));
        // why can I not pass Rc::downgrade(&testobserver1) to subscribe()?
        //let testdg1 = Rc::downgrade(&testobserver1);
        //let testdg2 = Rc::downgrade(&testobserver1);
        let testobserver2 = Rc::new(RefCell::new(TestObserver::new()));
        //let testdg3 = Rc::downgrade(&testobserver2);
        //let testobserver3 = Rc::new(RefCell::new(TestObserver::new()));
        //let testdg3 = Rc::downgrade(&testobserver3);
        //let testobserver4 = Rc::new(RefCell::new(TestObserver::new()));
        //let testdg4 = Rc::downgrade(&testobserver4);

        // create observable and subscriptions
        let mut testobservable: Observable = Observable::new();
        testobservable.subscribe(Event::Test1, &testobserver1);
        testobservable.subscribe(Event::Test2("test".to_string()), &testobserver1);
        testobservable.subscribe(Event::Test1, &testobserver2);
        //testobservable.subscribe(Event::Test1, testdg3);
        //testobservable.subscribe(Event::Test1, testdg4);

        // notify events
        testobservable.notify(Event::Test1);
        assert_eq!(testobserver1.borrow().t1_notifies, 1);
        assert_eq!(testobserver1.borrow().t2_notifies, 0);
        assert_eq!(testobserver2.borrow().t1_notifies, 1);
        assert_eq!(testobserver2.borrow().t2_notifies, 0);

        // todo: make this one work with a variable string (see struct comments above)
        testobservable.notify(Event::Test2("test".to_string()));
        assert_eq!(testobserver1.borrow().t1_notifies, 1);
        assert_eq!(testobserver1.borrow().t2_notifies, 1);
        assert_eq!(testobserver2.borrow().t1_notifies, 1);
        assert_eq!(testobserver2.borrow().t2_notifies, 0);
    }*/

    // run with 'cargo test -- --nocapture' to see println! output
    #[test]
    fn test_observer_integration() {
        let mut obsable = Observable::new();
        let obser1 = Observer{};
        let obser2 = Observer{};

        obsable.subscribe(&obser1);
        obsable.subscribe(&obser2);
        obsable.notify(Event{name:"testevent".to_string()});
    }
}

// notes
// mem::replace() can swap values of same type