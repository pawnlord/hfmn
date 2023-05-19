use core::fmt;
use std::{rc::*, cell::RefCell};


pub struct BinTree<T>{
    pub val : T,
    pub parent : Option<Rc<RefCell<BinTree<T>>>>,
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
        self.val.eq(&self2.val) && self.right.eq(&self2.right) && self.left.eq(&self2.left)
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
    fn print_tree_depth(&self, depth:i32){
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

pub fn add_tree<T>(tree: Rc<RefCell<BinTree<T>>>, child: Rc<RefCell<BinTree<T>>>, side: Side){
    child.borrow_mut().parent = Option::Some(tree.clone());
    match side {
        Side::Left => {tree.borrow_mut().left = Option::Some(child.clone());}
        Side::Right => {tree.borrow_mut().right = Option::Some(child.clone());}
    }
}
pub fn add_element<T>(tree: Rc<RefCell<BinTree<T>>>, val: T, side: Side) {
    let child : BinTree::<T>  = BinTree::<T>::new(val);
    add_tree(tree, Rc::new(RefCell::new(child)), side)
}
pub fn is_next_in_order<T>(tree: Rc<RefCell<BinTree<T>>>, is_stack_empty: bool) -> bool {
    let exists_right = tree.borrow_mut().right.is_some();

    return exists_right || !is_stack_empty;
}
pub fn has_parent<T>(tree: Rc<RefCell<BinTree<T>>>) -> bool {
    tree.borrow_mut().parent.is_some()
}

pub fn get_size<T>(tree: Rc<RefCell<BinTree<T>>>) -> u64{
    let left = if tree.borrow_mut().left.is_some(){
        get_size(tree.borrow_mut().left.as_ref().unwrap().clone())
    } else {0};
    let right = if tree.borrow_mut().right.is_some(){
        get_size(tree.borrow_mut().right.as_ref().unwrap().clone())
    } else {0};
    return left + right + 1;
}

pub fn create_from_orders<T: Eq + Clone + Copy + fmt::Display>(inorder: Vec<T>, mut preorder: Vec<T>) -> Rc<RefCell<BinTree<T>>> {
    assert!(preorder.len() >= 1);
    let t = preorder.remove(0);
    let curr_node: Rc<RefCell<BinTree<T>>> = BinTree::as_ref(t); 
    if inorder.len() <= 1 {
        return curr_node;
    } 
    let mut split_option: Option<usize> = None;
    for i in 0..inorder.len() {
        let t2 = inorder[i].clone();
        // println!("{} == {}", t, t2);
        if t == t2 {
            split_option = Some(i);
            break;
        }
    }
    if split_option.is_none(){
        panic!("I don't feel like making a result return, the trees don't match");
    }
    println!("{}", split_option.unwrap());
    let in_split = split_option.unwrap();
    let pre_split_option = preorder.iter().position(|&r| r == inorder[in_split-1]);
    if pre_split_option.is_none() {
        panic!("i expect thjis");
    }
    let pre_split = pre_split_option.unwrap() + 1;
    let left = create_from_orders(inorder[0..in_split].to_vec(), preorder[0..pre_split].to_vec());
    let right = create_from_orders(inorder[in_split+1..].to_vec(), preorder[pre_split..].to_vec());

    

    // println!("left 0..{}", in_split);
    // println!("left = {}", left.borrow_mut().val);
    // println!("right {}..", in_split+1);
    // println!("right = {}", right.borrow_mut().val);


    add_tree(curr_node.clone(), left, Side::Left);
    add_tree(curr_node.clone(), right, Side::Right);
    return curr_node;
}