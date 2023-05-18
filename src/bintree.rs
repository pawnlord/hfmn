use core::fmt;
use std::{rc::*, cell::RefCell};


pub struct BinTree<T>{
    pub val : T,
    pub parent : Option<Weak<RefCell<BinTree<T>>>>,
    pub right : Option<Rc<RefCell<BinTree<T>>>>,
    pub left : Option<Rc<RefCell<BinTree<T>>>>,
}
impl <T: Ord> PartialOrd for BinTree<T>{
    fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> { 
        self.val.partial_cmp(&rhs.val)
    }
}
impl <T: Ord> PartialEq for BinTree<T>{
    fn eq(&self, self2: &Self) -> bool {
        self.val.eq(&self2.val)
    }
}
impl <T: Ord> Eq for BinTree<T>{}

impl <T: Ord> Ord for BinTree<T>{
    fn cmp(&self, self2: &Self) -> std::cmp::Ordering {
        self.val.cmp(&self2.val)
    }
}
pub enum Side{
    Left,
    Right
}

impl<T> BinTree<T> {

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
}
impl<T: fmt::Display> BinTree<T> {
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
