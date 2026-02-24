#[derive(Debug)]
pub struct Rectangle {
    pub w: u32,
    pub h: u32,
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
