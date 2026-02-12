fn main() {
    let rec = Rectangle { w: 2, h: 3 };
    let area = rec.area();
    println!("Area is {area}");

    let rec2 = Rectangle { w: 3, h: 5 };
    let max_rec = rec.max(&rec2);
    println!("Max rec is max_rex: {max_rec:#?}");

    println!("Rectangle rec is: {rec:#?}");
    println!("Rectangle rec2 is: {rec2:#?}");

    let square = Rectangle::square(12);
    println!("This is a square: {square:#?}");
}

#[derive(Debug)]
struct Rectangle {
    w: u32,
    h: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.w * self.h
    }

    fn max(&self, other_rec: &Self) -> Self {
        let w = self.w.max(other_rec.w);
        let h = self.h.max(other_rec.h);
        Rectangle { w, h }
    }

    fn square(side: u32) -> Self {
        Self { w: side, h: side }
    }
}
