use bst::Tree;
fn main() {
    let t1 = Tree::new().insert(5);
    let t2 = t1.insert(3).insert(7);
    let t3 = t2.insert(4);

    println!("t3 contains 4? {}", t3.contains(&4)); // true
    println!("t3 contains 9? {}", t3.contains(&9)); // false
}
