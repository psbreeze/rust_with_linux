use std::rc::{Rc, Weak};
use std::cell::RefCell;

struct Person {
    next: RefCell<Option<Weak<Person>>>,
}

fn main() {
    let mut p1 = Rc::new(Person {
        next: RefCell::new(None),
    });

    let mut p2 = Rc::new(Person {
        next: RefCell::new(None),
    });

    let mut next = p1.next.borrow_mut();
    *next = Some(Rc::downgrade(&p2)); // weak 방식으로 p2 추가
    
    let mut next = p2.next.borrow_mut();
    *next = Some(Rc::downgrade(&p1)); // weak 방식으로 p1 추가

    println!("p1 RefCount: {} p2: RefCount: {}", 
        Rc::strong_count(&p1), Rc::strong_count(&p2));
}