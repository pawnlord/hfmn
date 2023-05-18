use core::fmt;
use std::{collections::HashMap, rc::Rc, cell::RefCell};

use crate::bintree::{*, self};

#[derive(PartialEq, Eq, PartialOrd)]
enum Node {
    Leaf(HuffmanNode),
    Branch(Rc<RefCell<BinTree<HuffmanNode>>>)
}

#[derive(PartialEq, Eq, PartialOrd, Clone, Copy)]
struct HuffmanNode{
    freq: u64,
    character: Option<u8>
}

pub struct HuffmanEncoding {
    bits: Rc<RefCell<Vec<bool>>>
}
pub struct HuffmanState{
    raw_data: Vec<u8>,
    decoding: Rc<RefCell<BinTree<HuffmanNode>>>,
    encoding: HashMap<u8, HuffmanEncoding>
}


impl HuffmanNode {
    pub fn empty(freq: u64) -> Self{
        Self{
            freq, 
            character: None}
    }
    pub fn new(freq: u64, c: u8) -> Self{
        Self{
            freq, 
            character: Some(c)}
    }
}

impl fmt::Display for HuffmanNode {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result { 
        if self.character.is_some(){
            write!(f, "({}, {})", self.freq, self.character.unwrap() as char)
        } else {
            write!(f, "({}, None)", self.freq)
        }
    }
}

impl Ord for HuffmanNode {
    fn cmp(&self, self2: &Self) -> std::cmp::Ordering {
        self.freq.cmp(&self2.freq)
    }
}

impl Node {
    fn get_freq(&self) -> u64 {
        match self {
            Node::Leaf(leaf) => {
                leaf.freq
            }

            Node::Branch(branch) => {
                branch.borrow_mut().val.freq
            }
        }
    }
    fn add_to_tree(&self, tree: Rc<RefCell<BinTree<HuffmanNode>>>, side: bintree::Side){
        match self{
            Node::Leaf(leaf) => {
                bintree::add_element(tree, leaf.clone(), side);
            }
            Node::Branch(branch) => {
                bintree::add_tree(tree, branch.clone(), side);
            }
        }
    }
    
    fn update_encoding(&self, encoding: &mut HashMap<u8, HuffmanEncoding>, flag: bool){
        match self {
            Node::Leaf(leaf) => {
                if leaf.character.is_some() {
                    update_char_encoding(leaf.character.unwrap(), encoding, flag);                
                }
            }
            Node::Branch(branch) => {
                update_tree_encoding(branch.clone(), encoding, flag);
            }
        }
    }
} 

fn update_char_encoding(ch: u8, encoding: &mut HashMap<u8, HuffmanEncoding>, flag: bool){
    if !encoding.contains_key(&ch) {
        encoding.insert(ch, HuffmanEncoding{bits: Rc::new(RefCell::new(vec!(flag)))});
    } else {
        let v = encoding.get(&ch).unwrap();
        v.bits.borrow_mut().insert(0, flag);
        encoding.insert(ch, HuffmanEncoding{bits: v.bits.clone()});
    }
}

fn update_tree_encoding(tree: Rc<RefCell<BinTree<HuffmanNode>>>, encoding: &mut HashMap<u8, HuffmanEncoding>, flag: bool){
    if tree.borrow_mut().val.character.is_some() {
        update_char_encoding(tree.borrow_mut().val.character.unwrap(), encoding, flag);
        return;
    }
    if tree.borrow_mut().left.is_some(){
       update_tree_encoding(tree.borrow_mut().left.as_ref().unwrap().clone(), encoding, flag); 
    }
    if tree.borrow_mut().right.is_some(){
       update_tree_encoding(tree.borrow_mut().right.as_ref().unwrap().clone(), encoding, flag); 
    }
}

impl Ord for Node {
    fn cmp(&self, self2: &Self) -> std::cmp::Ordering {
        let freq1: u64 = self.get_freq();
        let freq2: u64 = self2.get_freq();
        freq1.cmp(&freq2)
    }
}

fn generate_tree(mut list: Vec<Node>) -> (Rc<RefCell<BinTree<HuffmanNode>>>, HashMap<u8, HuffmanEncoding>) {
    let mut encoding = HashMap::<u8, HuffmanEncoding>::new();
    while list.len() > 1 {
        // Pop 2 values to add to tree structure
        let val1 = list.remove(0);
        let val2 = list.remove(0);


        let tree = BinTree::as_ref(HuffmanNode::empty(val1.get_freq() + val2.get_freq()));
        val1.add_to_tree(tree.clone(), Side::Right);
        val1.update_encoding(&mut encoding, true);
        val2.add_to_tree(tree.clone(), Side::Left);
        val2.update_encoding(&mut encoding, false);


        list.push(Node::Branch(tree.clone()));
        list.sort();
    }
    match list.get(0).unwrap() {
        Node::Leaf(_) => {}
        Node::Branch(b) => {b.borrow_mut().print_tree()}
    }
    match list.get(0) {
        Option::None => {
            panic!("Generate Tree failed, panicking")
        }
        Option::Some(x) => {
            match x {
                Node::Leaf(_) => {
                    panic!("Generate Tree failed, panicking")
                }
                Node::Branch(b) => {
                    (b.clone(), encoding)
                }
            }
        }
    }
}

impl HuffmanState{
    pub fn new(raw_data: Vec<u8>) -> Self {
        let mut map = HashMap::<u8, HuffmanNode>::new();
        for c in &raw_data {
            if !map.contains_key(&c) {
                map.insert(*c, HuffmanNode::new(1, *c));
            } else {
                let node = map.get(c).unwrap();
                map.insert(*c, HuffmanNode::new(node.freq + 1, node.character.unwrap()));
            }
        }
        let mut list = Vec::<Node>::new(); 
        for (_, node) in map {
            list.push(Node::Leaf(node));
        }
        list.sort();

        for l in &list {
            match l {
                Node::Leaf(leaf) => {println!("{}", leaf)}
                Node::Branch(_) => {}
            }
        }

        let (root, encoding) = generate_tree(list);
        

        Self { raw_data: raw_data, decoding: root, encoding}

    }

}