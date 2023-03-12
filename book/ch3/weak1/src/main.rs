use std::rc::Rc;
use std::cell::RefCell;

struct Person {
    next: RefCell<Option<Rc<Person>>>,
}

fn main() {
    let mut p1 = Rc::new(Person {
        next: RefCell::new(None),
    });

    let mut p2 = Rc::new(Person {
        next: RefCell::new(None),
    });

    let mut next = p1.next.borrow_mut();
    *next = Some(p2.clone()); // p1뒤에 p2를 추가
    
    let mut next = p2.next.borrow_mut();
    *next = Some(p1.clone()); // p2뒤에 p1를 추가

    println!("p1 RefCount: {} p2: RefCount: {}", 
        Rc::strong_count(&p1), Rc::strong_count(&p2));
}