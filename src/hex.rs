use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Clone, Copy, Debug, Default, Hash, Eq, PartialEq)]
pub struct Hex {
    pub q: i32,
    pub r: i32
}
impl Hex {
    pub fn new(q: i32, r: i32) -> Hex {
        Hex { q, r }
    }
}

impl Add for Hex {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        return Hex::new(self.q + other.q, self.r + other.r);
    }
}
impl AddAssign for Hex {
    fn add_assign(&mut self, other: Self) {
        *self = Self{q: self.q + other.q, r: self.r + other.r};
    }
}
impl Sub for Hex {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        return Hex::new(self.q - other.q, self.r - other.r)
    }
}
impl SubAssign for Hex {
    fn sub_assign(&mut self, other: Self) {
        *self = Self{q: self.q - other.q, r: self.r - other.r};
    }
}