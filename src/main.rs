extern crate sciter;

use std::rc::Rc;
use std::cell::RefCell;

struct Thought{
    parent: RefCell<Vec<Rc<Thought>>>,
    childs: RefCell<Vec<Rc<Thought>>>,
    topic:  String,
}

impl Thought {
    fn new(self, topic: &str) -> Rc<Thought> {
        let s = Rc::new(self);
        let t = Rc::new(Thought{ parent: RefCell::new(vec![s.clone()]),
                                 childs: RefCell::new(Vec::new()),
                                 topic: topic.to_string()
        });
        s.childs.borrow_mut().push(t.clone());
        t
    }
}

fn main() {
    let root = Thought{ parent: Rc::new(None), childs: RefCell::new(Vec::new()), "root".to_string() };
    let mut frame = sciter::Window::new();
    frame.load_file("own_brain.htm");
    frame.run_app(true);
}
