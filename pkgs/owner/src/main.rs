use std::rc::Rc;

fn main() {
    let mut a = [0, 1, 2, 3];
    let y = a[2];
    let x = &mut a[1];
    *x += 1;
    *x += y;
    println!("{a:?}");

    let a = Box::new([0; 1_000]);

    println!("{}", a.len());
    move_onwership(a);

    let mut vec = vec!['a', 'b', 'c'];
    ascii_capitalize(&mut vec);

    let b = 5;
    let c = &b;
    println!("Address stored in c is:{:p}", c);
    println!("c is: {c}");

    let the_string = return_a_string_fix1();
    println!("Value of the String is: {}", the_string);
}

fn move_onwership(_x: Box<[i32; 1000]>) {
    // Do nothing here!
}

fn ascii_capitalize(v: &mut Vec<char>) {
    let c = &v[0];
    if c.is_ascii_lowercase() {
        let up = c.to_ascii_uppercase();
        v[0] = up;
    } else {
        println!("Already captlized: {:?}", v);
    }
}

// fn return_a_string_err() -> &String {
//     let s = String::from("Hello world");
//     &s
// }

fn return_a_string_fix1() -> String {
    String::from("Hello World")
}

fn return_a_string_fix2() -> Rc<String> {
    let s = Rc::new(String::from("Hello world"));
    Rc::clone(&s)
}

fn return_a_string_fix3(output: &mut String) {
    output.replace_range(.., "Hello World");
}
