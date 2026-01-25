fn main() {
    println!("Hello, world!");
    another_function(32);
    print_measurement_label(16, 'h');
    let x = return_some_value();
    println!("The value returned from function is {x}.");
    let x = plus_one(x);
    println!("The return value of func 'plus_one' is {x}.")
}

fn another_function(x: i32) {
    println!("Messege from another function.");
    println!("The value of paramater x is {x}.");
}

fn print_measurement_label(value: i32, label: char) {
    println!("The measurement is: {value}{label}.");
}

fn return_some_value() -> i32 {
    let x = 5;
    x + 1
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
