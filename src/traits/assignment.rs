use crate::Yotta;
use core::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

impl AddAssign for Yotta {
    fn add_assign(&mut self, other: Self) {
        *self = self.add_impl(&other);
    }
}

impl SubAssign for Yotta {
    fn sub_assign(&mut self, other: Self) {
        *self = self.sub_impl(&other);
    }
}

impl MulAssign for Yotta {
    fn mul_assign(&mut self, other: Self) {
        *self = self.mul_impl(&other);
    }
}

impl DivAssign for Yotta {
    fn div_assign(&mut self, other: Self) {
        *self = self.div_impl(&other).unwrap();
    }
}
