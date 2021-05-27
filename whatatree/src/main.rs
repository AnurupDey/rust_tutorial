use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

#[derive(Debug)]
struct Node {
    value: i32,
    children: Vec<Rc<RefCell<Node>>>,
}

fn main() {
    let root = Rc::new(RefCell::new(Node {
        value: 30,
        children: Vec::new(),
    }));

    root.borrow_mut().children.push(Rc::new(RefCell::new(Node {
        value: 40,
        children: Vec::new(),
    })));
    root.borrow_mut().children.push(Rc::new(RefCell::new(Node {
        value: 22,
        children: Vec::new(),
    })));

    let a = &root.borrow().children[0];
    let b = &root.borrow().children[1];

    a.borrow_mut().value = b.borrow().value;

    println!("{:#?}", root);
}
