use crate::Yotta;
use core::ops::Neg;

impl Neg for Yotta {
    type Output = Self;
    fn neg(self) -> Self {
        Yotta {
            negative: !self.negative,
            ..self
        }
    }
}
