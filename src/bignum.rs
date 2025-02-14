use crate::digits::*;
use alloc::{vec, vec::Vec};

#[repr(C)]
#[derive(Clone)]
pub struct Yotta {
    pub mantissa: Vec<u8>,
    pub exponent: i32,
    pub bit_width: u32,
    pub negative: bool,
}

/// Yotta Arbitrary Precision
///
/// # Fields
/// * `mantissa` - Vector of single digits stored as bytes (0-9)
/// * `exponent` - Power of 10 exponent
/// * `bit_width` - Maximum number of bits used to represent the number
/// * `negative` - Boolean flag indicating if the number is negative
///
/// # Examples
/// Basic usage:
/// ```
/// use yotta::Yotta;
/// let num1 = Yotta::new("123", 32);
/// let num2 = Yotta::new("456", 32);
/// let sum = num1.clone() + num2.clone();
/// let diff = num2.clone() - num1.clone();
/// assert_eq!(sum, Yotta::new("579", 32));
/// assert_eq!(diff, Yotta::new("333", 32));
/// ```
impl Yotta {
    pub fn new(value: &str, bit_width: u32) -> Self {
        let mut negative = false;
        let value = if let Some(stripped) = value.strip_prefix('-') {
            negative = true;
            stripped
        } else {
            value
        };
        let capacity = (bit_width / 8) as usize;
        let bytes = value.as_bytes();
        let len = bytes.len().min(capacity);
        let mantissa = bytes[..len].iter().map(|b| b - b'0').collect::<Vec<u8>>();
        Yotta {
            mantissa,
            exponent: 0,
            bit_width,
            negative,
        }
    }

    fn from_digits_with_width(mut digits: Vec<u8>, negative: bool, bit_width: u32) -> Self {
        let non_zero_index = digits
            .iter()
            .position(|&x| x != 0)
            .unwrap_or(digits.len() - 1);
        digits = digits[non_zero_index..].to_vec();
        let is_zero = digits.len() == 1 && digits[0] == 0;
        Yotta {
            mantissa: digits,
            exponent: 0,
            bit_width,
            negative: if is_zero { false } else { negative },
        }
    }

    pub fn add_impl(&self, other: &Yotta) -> Yotta {
        if self.negative == other.negative {
            let sum = add_digits(&self.mantissa, &other.mantissa);
            Yotta::from_digits_with_width(sum, self.negative, self.bit_width)
        } else {
            let cmp = compare_digit_slices(&self.mantissa, &other.mantissa);
            match cmp {
                0 => Yotta::from_digits_with_width(vec![0], false, self.bit_width),
                x if x > 0 => {
                    let diff = sub_digits(&self.mantissa, &other.mantissa);
                    Yotta::from_digits_with_width(diff.0, self.negative, self.bit_width)
                }
                _ => {
                    let diff = sub_digits(&other.mantissa, &self.mantissa);
                    Yotta::from_digits_with_width(diff.0, other.negative, self.bit_width)
                }
            }
        }
    }

    pub fn sub_impl(&self, other: &Yotta) -> Yotta {
        let mut neg_other = other.clone();
        neg_other.negative = !other.negative;
        self.add_impl(&neg_other)
    }

    pub fn mul_impl(&self, other: &Yotta) -> Yotta {
        let prod = mul_digits(&self.mantissa, &other.mantissa);
        let new_bw = self.bit_width.max(other.bit_width);
        Yotta::from_digits_with_width(prod, self.negative ^ other.negative, new_bw)
    }

    pub fn div_impl(&self, other: &Yotta) -> Yotta {
        let quot = div_digits(&self.mantissa, &other.mantissa);
        let new_bw = self.bit_width.max(other.bit_width);
        Yotta::from_digits_with_width(quot, self.negative ^ other.negative, new_bw)
    }
}
