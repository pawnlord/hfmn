use core::fmt;
use std::{collections::HashMap};

use crate::bintree::*;

#[derive(PartialEq, Eq, PartialOrd)]
struct HuffmanNode{
    freq: u64,
    character: Option<u8>
}
pub struct HuffmanState{
    raw_data: Vec<u8>,
    root: BinTree<HuffmanNode>
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
    pub fn inc_freq(&mut self){
        self.freq += 1;
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
        let mut list = Vec::<HuffmanNode>::new(); 
        for (_, node) in map {
            list.push(node);
        }
        list.sort();

        for l in list {
            println!("{}", l);
        }

        let root = BinTree::new(HuffmanNode::empty(0));
        Self { raw_data: raw_data, root: root }

    }
}