use crate::Yotta;
use alloc::{string::String, vec};
use core::fmt::{Debug, Display, Formatter, Result};
use core::str::FromStr;

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
        write!(
            f,
            "Yotta {{ mantissa: {}, exponent: {}, bit_width: {}, negative: {} }}",
            s, self.exponent, self.bit_width, self.negative
        )
    }
}

impl FromStr for Yotta {
    type Err = core::fmt::Error;
    fn from_str(s: &str) -> core::result::Result<Yotta, Self::Err> {
        let negative = s.starts_with('-');
        let s = if negative { &s[1..] } else { s };
        let mut res = Yotta {
            mantissa: vec![0; s.len()],
            exponent: 0,
            bit_width: (s.len() as u32) * 8,
            negative,
        };
        res.mantissa[..s.len()].copy_from_slice(s.as_bytes());
        core::result::Result::Ok(res)
    }
}
