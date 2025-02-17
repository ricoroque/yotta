use crate::digits::*;
use alloc::{string::String, vec::Vec, vec};

#[repr(C)]
#[derive(Clone)]
pub struct Yotta {
    pub mantissa: Vec<u8>,
    pub exponent: i32,
    pub bit_width: u32,
    pub negative: bool,
}

impl Yotta {
    pub fn new(value: &str, bit_width: u32) -> Self {
        let mut negative = false;
        let mut value = value.trim();
        if let Some(stripped) = value.strip_prefix('-') {
            negative = true;
            value = stripped;
        }

        let mut exponent = 0;
        let mut clean = String::new();
        if let Some(dot_index) = value.find('.') {
            // Remove the decimal point and record fractional length.
            let (int_part, frac_part) = value.split_at(dot_index);
            // frac_part starts with '.', so skip it.
            let frac_part = &frac_part[1..];
            exponent = -(frac_part.len() as i32);
            clean.push_str(int_part);
            clean.push_str(frac_part);
        } else {
            clean.push_str(value);
        }

        // Limit capacity based on bit_width.
        let capacity = (bit_width / 8) as usize;
        let bytes = clean.as_bytes();
        let len = bytes.len().min(capacity);
        let mantissa = bytes[..len].iter().map(|b| b - b'0').collect::<Vec<u8>>();
        Yotta {
            mantissa,
            exponent,
            bit_width,
            negative,
        }
    }

    // Helper to scale a mantissa by appending zeros.
    fn scale_mantissa(mantissa: &Vec<u8>, scale: u32) -> Vec<u8> {
        let mut scaled = mantissa.clone();
        for _ in 0..scale {
            scaled.push(0);
        }
        scaled
    }

    // Remove leading zeros.
    fn from_digits_with_width(mut digits: Vec<u8>, negative: bool, exponent: i32, bit_width: u32) -> Self {
        let non_zero_index = digits.iter().position(|&x| x != 0).unwrap_or(digits.len()-1);
        digits = digits[non_zero_index..].to_vec();
        let is_zero = digits.len() == 1 && digits[0] == 0;
        Yotta {
            mantissa: digits,
            exponent,
            bit_width,
            negative: if is_zero { false } else { negative },
        }
    }

    pub fn add_impl(&self, other: &Yotta) -> Yotta {
        // Align exponents by scaling the mantissa with the higher exponent (less fractional part)
        let target_exponent = self.exponent.min(other.exponent);
        let diff_self = (self.exponent - target_exponent) as u32;
        let diff_other = (other.exponent - target_exponent) as u32;
        let m_self = Self::scale_mantissa(&self.mantissa, diff_self);
        let m_other = Self::scale_mantissa(&other.mantissa, diff_other);

        // When signs match, use addition; otherwise use subtraction.
        if self.negative == other.negative {
            let sum = add_digits(&m_self, &m_other);
            Yotta::from_digits_with_width(sum, self.negative, target_exponent, self.bit_width)
        } else {
            let cmp = compare_digit_slices(&m_self, &m_other);
            match cmp {
                0 => Yotta::from_digits_with_width(vec![0], false, target_exponent, self.bit_width),
                x if x > 0 => {
                    let diff = sub_digits(&m_self, &m_other);
                    Yotta::from_digits_with_width(diff.0, self.negative, target_exponent, self.bit_width)
                }
                _ => {
                    let diff = sub_digits(&m_other, &m_self);
                    Yotta::from_digits_with_width(diff.0, other.negative, target_exponent, self.bit_width)
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
        let new_exponent = self.exponent + other.exponent;
        let new_bw = self.bit_width.max(other.bit_width);
        Yotta::from_digits_with_width(prod, self.negative ^ other.negative, new_exponent, new_bw)
    }

    pub fn div_impl(&self, other: &Yotta) -> Yotta {
        // If either number has fractional digits, scale the numerator
        let scale = if self.exponent < 0 || other.exponent < 0 {
            // Use the absolute value of the smallest exponent as the scale factor.
            (-self.exponent.min(other.exponent)) as u32
        } else {
            0
        };
        let scaled_numer = Self::scale_mantissa(&self.mantissa, scale);
        let quot = div_digits(&scaled_numer, &other.mantissa);
        let new_exponent = self.exponent - other.exponent - (scale as i32);
        Yotta::from_digits_with_width(quot, self.negative ^ other.negative, new_exponent, self.bit_width)
    }
}
