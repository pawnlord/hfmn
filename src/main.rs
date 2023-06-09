use std::io::{self, BufRead};

mod huffman;
mod bintree;

fn main() {
    let root  = bintree::BinTree::as_ref(0);
    println!("Test 1: ########");
    root.borrow().print_tree();
    bintree::add_element(root.clone(), 4, bintree::Side::Right);
    bintree::add_element(root.clone(), 5, bintree::Side::Left);
    println!("Test 2: ########");
    root.borrow().print_tree();
    let new_tree = bintree::BinTree::as_ref(2);
    bintree::add_tree(new_tree.clone(), root, bintree::Side::Left);
    println!("Test 3: ########");
    new_tree.borrow().print_tree();
    

    let mut data : Vec<u8> = Vec::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line_str = line.unwrap_or("".to_string()) + "\n";
        for c in line_str.as_bytes() {
            data.push(c.clone());
        }
    }

    println!("size in: {}", data.len());

    let hfmn = huffman::HuffmanState::new(data);
    let compressed_data = hfmn.compress();
    println!("Size of compressed data: {}", compressed_data.len());
    let decompressed_data = hfmn.decompress(compressed_data.clone());
    for u in &decompressed_data {
        print!("{}", *u as char);
    }
    println!("Size of decompressed data: {}", decompressed_data.len());
    {
        let file = std::fs::File::create("hello.txt");
        hfmn.save_to_file(&file.unwrap());
    }
    {
        let file = std::fs::File::open("hello.txt");
        let (hfmn2, raw_data) = huffman::HuffmanState::load_from_file(&mut file.unwrap());
        // for u in raw_data {
        //     print!("{}", u as char);
        // }
        println!("{} == {}: {}", raw_data.len(), compressed_data.len(), raw_data == compressed_data);
    }
}
