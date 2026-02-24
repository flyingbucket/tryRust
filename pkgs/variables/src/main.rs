const GLOBAL_SPEC_CONST: &str = "A global spec const";
fn main() {
    let x = 5;
    println!("The value of x is {x}");
    let x = x + 1;
    println!("The value of x shadowed in outer spec is {x}");
    {
        let x = x + 1;
        println!("The value of x shadowed in inner spec is {x}");
    }

    println!("The value of x back in outer spec is {x}");
    println!("\n");
    println!("Stuff about const:");
    println!("Const in global spec is {GLOBAL_SPEC_CONST}");
    const THREE_HOURS_IN_SECOUNDS: i32 = 3 * 60 * 60;
    println!("Const THREE_HOURS_IN_SECOUNDS is {THREE_HOURS_IN_SECOUNDS}");
}
