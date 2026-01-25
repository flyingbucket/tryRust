fn main() {
    println!("There are three kinds of loops: \n\tloop,\n\tfor loop,\n\twhile loop");

    println!("1. loop");
    let mut counter = 0;
    let res: u64 = loop {
        counter += 1;
        if counter == 10 {
            break counter * counter;
        }
    };
    println!("when break from a loop, res is {res}")
}
