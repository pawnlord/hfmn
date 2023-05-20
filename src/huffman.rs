use core::fmt;
use std::{collections::HashMap, rc::Rc, cell::RefCell, io::{Write, Read, Seek}};

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

fn create_encoding_from_decoding(decoding: Rc<RefCell<BinTree<HuffmanNode>>>) -> HashMap<u8, HuffmanEncoding> {
    let mut encoding = HashMap::<u8, HuffmanEncoding>::new();
    let mut stack = Vec::<(Rc<RefCell<BinTree<HuffmanNode>>>, Vec::<bool>)>::new();
    let mut curr_encoding = Vec::<bool>::new();    
    let mut curr_node = decoding.clone();
    while !stack.is_empty() || curr_node.borrow_mut().left.is_some() {
        if curr_node.borrow_mut().val.character.is_some() {
            encoding.insert(curr_node.borrow_mut().val.character.unwrap(),
                HuffmanEncoding{bits: Rc::new(RefCell::new(curr_encoding.clone()))});
        }
        if curr_node.borrow_mut().left.is_some() {
            if curr_node.borrow_mut().right.is_some(){
                let mut clone = curr_encoding.clone();
                clone.push(true);
                stack.push((
                    curr_node.borrow_mut().right.as_ref().unwrap().clone(),
                    clone
                ));
            }
            let right = curr_node.borrow_mut().right.as_ref().unwrap().clone();
            curr_node = right;
            curr_encoding.push(false);
        } else {
            // There is something in the stack
            (curr_node, curr_encoding) = stack.pop().unwrap();
        }
    }
    return encoding;
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

    pub fn compress(&self) -> Vec<u8> {
        // First pass, slow and inefficient
        let mut raw_flags = Vec::<u8>::new();
        for c in &self.raw_data{
            let encoding_option = self.encoding.get(c);
            if encoding_option.is_none(){
                continue;
            }
            let encoding = encoding_option.unwrap();
            for flag in encoding.bits.borrow_mut().clone() {
                raw_flags.push(if flag {1} else {0});
            }
        }
        let mut compressed_data = Vec::<u8>::new();
        let mut bit = 0;
        let mut current = 0;
        compressed_data.push(0);
        for flag in raw_flags {
            compressed_data[current] |= flag << bit;
            bit += 1;
            if bit == 8 {
                bit = 0;
                current += 1;
                compressed_data.push(0);
            }
        }

        return compressed_data;
    }
    pub fn decompress(&self, compressed : Vec<u8>) -> Vec<u8> {
        let mut bit = 0;
        let mut current_node = self.decoding.clone();
        let mut uncompressed: Vec<u8> = Vec::new();
        for c in compressed {
            while bit < 8 {
                let flag = c & (1<<bit);
                if flag != 0 {
                    if current_node.borrow_mut().right.is_some() {
                        let temp = current_node.borrow_mut().right.as_ref().unwrap().clone();
                        current_node = temp;
                    }
                } else {
                    if current_node.borrow_mut().left.is_some() {
                        let temp = current_node.borrow_mut().left.as_ref().unwrap().clone();
                        current_node = temp;
                    }
                }
                if current_node.borrow_mut().val.character.is_some() {
                    uncompressed.push(current_node.borrow_mut().val.character.unwrap());
                    current_node = self.decoding.clone();
                }
                bit += 1;
            }
            bit = 0;
        }
        return uncompressed;
    }
    pub fn save_to_file(&self, mut file: &std::fs::File){
        // Find lowest left node
        let mut curr_node: Rc<RefCell<BinTree<HuffmanNode>>> = self.decoding.clone();
        let mut stack = Vec::<Rc<RefCell<BinTree<HuffmanNode>>>>::new();

        while curr_node.borrow_mut().left.is_some(){
            stack.push(curr_node.clone());
            let left = curr_node.borrow_mut().left.as_ref().unwrap().clone();
            curr_node = left; 
        }
        let offset = (5 * bintree::get_size(self.decoding.clone())) + 8;
        
        let mut offset_u8 = Vec::<u8>::new();
        for i in 0..8 {
            offset_u8.push(((offset >> (8*i)) & 0xFF) as u8);
        }
        file.write(offset_u8.as_slice());
        
        write_node(file, curr_node.clone());

        while bintree::is_next_in_order(curr_node.clone(), stack.is_empty()) {
            if curr_node.borrow_mut().right.is_some() {
                let right = curr_node.borrow_mut().right.as_ref().unwrap().clone();
                curr_node = right;
                while curr_node.borrow_mut().left.is_some(){
                    stack.push(curr_node.clone());
                    let left = curr_node.borrow_mut().left.as_ref().unwrap().clone();
                    curr_node = left; 
                }
            } else {
                curr_node = stack.pop().unwrap().clone();
            }
            // Write current node
            write_node(file, curr_node.clone());
        }
        curr_node = self.decoding.clone();
        // not needed 
        stack.clear();
        
        if curr_node.borrow_mut().right.is_some(){
            stack.push(curr_node.borrow_mut().right.as_ref().unwrap().clone());
        }
        
        while !stack.is_empty() || curr_node.borrow_mut().left.is_some() {
            write_node(file, curr_node.clone());
            if curr_node.borrow_mut().left.is_some() {
                if curr_node.borrow_mut().right.is_some() {
                    stack.push(curr_node.borrow_mut().right.as_ref().unwrap().clone());
                }
                let left = curr_node.borrow_mut().left.as_ref().unwrap().clone();
                curr_node = left;
            } else {
                curr_node = stack.pop().unwrap();
            }
        }
        write_node(file, curr_node.clone());
        // save compressed data
        let data = self.compress();
        file.write(data.as_slice());
    }

    pub fn load_from_file(mut file: &mut std::fs::File) -> (Self, Vec<u8>){
        let mut inorder = Vec::<HuffmanNode>::new();
        let mut preorder = Vec::<HuffmanNode>::new();
        let offset = read_u64(&mut file);
        println!("{}", Seek::seek(&mut file, std::io::SeekFrom::Current(0)).unwrap());
        while Seek::seek(&mut file, std::io::SeekFrom::Current(0)).unwrap() < offset {
            inorder.push(read_node(&mut file));
        }
        // 2*size of  tree + offset = (2*(size of tree + offset)) - offset  
        while Seek::seek(&mut file, std::io::SeekFrom::Current(0)).unwrap() < (2*offset)-8 {
            preorder.push(read_node(&mut file));
        }
        let tree = create_from_orders(inorder, preorder);
        tree.borrow_mut().print_tree();
        let encoding = create_encoding_from_decoding(tree.clone());
        let mut raw_data_u8 = Vec::<u8>::new();
        file.read_to_end(&mut raw_data_u8);
        let mut hfmn = Self{
            raw_data: Vec::new(),
            encoding: encoding,
            decoding: tree
        };
        let raw_data = hfmn.decompress(raw_data_u8.to_vec());
        println!("Size of decompressed data: {}", raw_data.len());
        hfmn.raw_data = raw_data.clone();
        return (hfmn, raw_data_u8);
    }
}
fn write_node(mut file: &std::fs::File, curr_node: Rc<RefCell<BinTree<HuffmanNode>>>) {
    let mut freq = Vec::<u8>::new();
    let mut char = Vec::<u8>::new();
    for i in 0..4 {
        freq.push(((curr_node.borrow_mut().val.freq >> (8*i)) & 0xFF) as u8);
    }
    char.push(if curr_node.borrow_mut().val.character.is_some() {
            *curr_node.borrow_mut().val.character.as_ref().unwrap()
        } else {0});
    file.write(freq.as_slice());
    file.write(char.as_slice());

}
fn read_u64(file: &mut std::fs::File) -> u64 {
    let mut integer_u8 = vec![0u8; 8];
    let result = file.read_exact(&mut integer_u8);
    let mut integer: u64 = 0;
    assert!(result.is_ok());
    for i in 0..8{
        integer |= (integer_u8[i] as u64) << (i*8);
    }
    integer
}
fn read_node(file: &mut std::fs::File) -> HuffmanNode {
    let mut freq_u8 = vec![0u8; 4];
    let mut result = file.read_exact(&mut freq_u8);
    let mut freq: u32 = 0;
    assert!(result.is_ok());
    for i in 0..4{
        freq |= (freq_u8[i] as u32) << (i*8);
    }
    let mut char_u8 = vec![0u8; 1];
    result = file.read_exact(&mut char_u8);
    assert!(result.is_ok());
    if char_u8[0] == 0 {
        HuffmanNode::empty(freq as u64)
    } else {
        HuffmanNode::new(freq as u64, char_u8[0])
    }
}