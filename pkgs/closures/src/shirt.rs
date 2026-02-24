#[derive(Debug, PartialEq, Clone)]
pub enum ShirtColor {
    Red,
    Blue,
}

#[derive(Debug)]
pub struct Inventory {
    pub shirts: Vec<ShirtColor>,
}

impl Inventory {
    pub fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut blue_num = 0;
        let mut red_num = 0;
        for shirt in &self.shirts {
            match shirt {
                ShirtColor::Red => red_num += 1,
                ShirtColor::Blue => blue_num += 1,
            }
        }
        if red_num > blue_num {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}
