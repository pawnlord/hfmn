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
}
