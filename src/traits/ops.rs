use crate::Yotta;
use core::ops::{Add, Div, Mul, Sub, Neg};

impl Add for Yotta {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        self.add_impl(&other)
    }
}

impl Sub for Yotta {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        self.sub_impl(&other)
    }
}

impl Mul for Yotta {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        self.mul_impl(&other)
    }
}

impl Div for Yotta {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        self.div_impl(&other).unwrap()
    }
}

impl Neg for Yotta {
    type Output = Self;
    fn neg(self) -> Self {
        Yotta {
            negative: !self.negative,
            ..self
        }
    }
}
