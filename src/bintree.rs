use core::fmt;
use std::{rc::*, cell::RefCell};


pub struct BinTree<T : fmt::Display> {
    parent : Option<Weak<RefCell<BinTree<T>>>>,
    val : T,
    right : Option<Rc<RefCell<BinTree<T>>>>,
    left : Option<Rc<RefCell<BinTree<T>>>>,
}

pub enum Side{
    Left,
    Right
}

impl<T: fmt::Display> BinTree<T> {

    pub fn new(val: T) -> Self {
        Self{parent: None,
            val,
            right: None,
            left: None,
        }
    }
    pub fn as_ref(val: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self { parent: None, 
            val, 
            right: None, 
            left: None }))
    }
    pub fn print_tree(&self){
        self.print_tree_depth(0)
    }
    pub fn print_tree_depth(&self, depth:i32){
        for _ in 0..depth {
            print!("-")
        }
        println!("{}", self.val);
        if self.left.is_some() {
            print!("L");
            self.left.as_ref().unwrap().borrow().print_tree_depth(depth + 1)
        }
        if self.right.is_some() {
            print!("R");
            self.right.as_ref().unwrap().borrow().print_tree_depth(depth + 1)
        }
    }
}

pub fn add_tree<T: fmt::Display>(tree: Rc<RefCell<BinTree<T>>>, child: Rc<RefCell<BinTree<T>>>, side: Side){
    child.borrow_mut().parent = Option::Some(Rc::downgrade(&tree.clone()));
    match side {
        Side::Left => {tree.borrow_mut().left = Option::Some(child.clone());}
        Side::Right => {tree.borrow_mut().right = Option::Some(child.clone());}
    }
}
pub fn add_element<T: fmt::Display>(tree: Rc<RefCell<BinTree<T>>>, val: T, side: Side) {
    let child : BinTree::<T>  = BinTree::<T>::new(val);
    add_tree(tree, Rc::new(RefCell::new(child)), side)
}
