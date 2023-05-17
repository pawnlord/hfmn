mod huffman;

fn main() {
    let root  = huffman::BinTree::as_ref(0);
    println!("Test 1: ########");
    root.borrow().print_tree();
    huffman::BinTree::add_element(root.clone(), 4, huffman::Side::Right);
    huffman::BinTree::add_element(root.clone(), 5, huffman::Side::Left);
    println!("Test 2: ########");
    root.borrow().print_tree();
    let new_tree = huffman::BinTree::as_ref(2);
    huffman::BinTree::add_tree(new_tree.clone(), root, huffman::Side::Left);
    println!("Test 3: ########");
    new_tree.borrow().print_tree();
}
