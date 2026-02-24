fn main() {
    let num = 7;
    if num < 5 {
        println!("contidion is true");
    } else {
        println!("contidion is false");
    }

    if num == 2 {
        println!("num is 2")
    } else if num == 3 {
        println!("num is 3")
    } else {
        println!("num is not 2 or 3")
    }
    if num % 4 == 0 {
        println!("num is divisible by 4");
    } else if num % 3 == 0 {
        println!("num is divisible by 3");
    } else if num % 2 == 0 {
        println!("num is divisible by 2");
    } else {
        println!("num is not divisible by 4, 3, or 2");
    }

    let cond = false;
    let num1 = if cond { 1 } else { 0 };
    println!("cond is: {cond}, num is: {num1}")
}
