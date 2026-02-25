use std::cmp::Ordering;
use std::fmt;

fn main() {
    let points = [
        Point(0.0, 0.0),
        Point(0.0, 1.0),
        Point(12.0, -3.34),
        Point(17.0, -0.28),
    ];
    let largest_mod = largest(&points);
    println!("The farest point from origin is {largest_mod}");
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut lgt = &list[0];
    for element in list {
        if element > lgt {
            lgt = element
        }
    }
    lgt
}

#[derive(PartialEq)]
struct Point(f32, f32);

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let left_len = self.0 * self.0 + self.1 * self.1;
        let right_len = other.0 * other.0 + other.1 * other.1;

        left_len.partial_cmp(&right_len)
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Point({}, {})", self.0, self.1)
    }
}
