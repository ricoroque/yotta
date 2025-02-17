use crate::Yotta;
use alloc::{string::String, vec};
use core::fmt::{Debug, Display, Formatter, Result};

impl Display for Yotta {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut mantissa = String::new();
        for &digit in &self.mantissa {
            mantissa.push((digit + b'0') as char);
        }
        let s = mantissa.trim_end_matches('\0');
        if self.negative && s != "0" {
            write!(f, "-{}", s)
        } else {
            write!(f, "{}", s)
        }
    }
}

impl Debug for Yotta {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let s: String = self.mantissa.iter().map(|&d| (d + b'0') as char).collect();
        let f_s: String = if self.frac_part.is_empty() {
            "0".into()
        } else {
            self.frac_part.iter().map(|&d| (d + b'0') as char).collect()
        };
        write!(
            f,
            "Yotta {{ mantissa: {}, frac_part: {}, exponent: {}, bit_width: {}, negative: {} }}",
            s, f_s, self.exponent, self.bit_width, self.negative
        )
    }
}
