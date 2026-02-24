fn main() {
    println!("Hello, world!");
}

fn largest<T>(list: &[T]) -> &T {
    let mut lgt = &list[0];
    for element in list {
        if element > lgt {
            lgt = element
        }
    }
    lgt
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
