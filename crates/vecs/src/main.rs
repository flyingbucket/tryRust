fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    let third = &v[2];
    println!("The third value is {third}");

    let sixth_from_get = v.get(5);
    match sixth_from_get {
        Some(sixth_from_get) => println!("The sixth value is {sixth_from_get}"),
        None => println!("There is no sixth value yet."),
    }

    v.push(6);

    let sixth_from_get = v.get(5);
    match sixth_from_get {
        Some(sixth_from_get) => println!("The sixth value is {sixth_from_get}"),
        None => println!("There is no third value."),
    }

    for x in &mut v {
        *x += 100;
    }
    println!("Modified vector is {v:?}")
}
