use crate::Yotta;
use core::cmp::{PartialEq, PartialOrd};

impl PartialEq for Yotta {
    fn eq(&self, other: &Self) -> bool {
        self.mantissa == other.mantissa && self.negative == other.negative
    }
}

impl PartialOrd for Yotta {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        // Compare signs first.
        if self.negative != other.negative {
            return if self.negative {
                Some(core::cmp::Ordering::Less)
            } else {
                Some(core::cmp::Ordering::Greater)
            };
        }
        let cmp = self.mantissa.cmp(&other.mantissa);
        if self.negative {
            Some(cmp.reverse())
        } else {
            Some(cmp)
        }
    }
}
