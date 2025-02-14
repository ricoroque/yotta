use alloc::{vec, vec::Vec};

pub fn trim_leading_zeros(mut digits: Vec<u8>) -> Vec<u8> {
    if let Some(idx) = digits.iter().position(|&x| x != 0) {
        digits = digits[idx..].to_vec();
    }
    if digits.is_empty() {
        vec![0]
    } else {
        digits
    }
}

pub fn compare_digit_slices(a: &[u8], b: &[u8]) -> i32 {
    match a.len().cmp(&b.len()) {
        core::cmp::Ordering::Greater => 1,
        core::cmp::Ordering::Less => -1,
        core::cmp::Ordering::Equal => {
            for (&x, &y) in a.iter().zip(b.iter()) {
                match x.cmp(&y) {
                    core::cmp::Ordering::Greater => return 1,
                    core::cmp::Ordering::Less => return -1,
                    core::cmp::Ordering::Equal => continue,
                }
            }
            0
        }
    }
}

pub fn add_digits(a: &[u8], b: &[u8]) -> Vec<u8> {
    let mut i = a.len() as isize - 1;
    let mut j = b.len() as isize - 1;
    let mut carry = 0;
    let mut res = Vec::with_capacity(a.len().max(b.len()) + 1);

    while i >= 0 || j >= 0 || carry != 0 {
        let d1 = if i >= 0 { a[i as usize] } else { 0 };
        let d2 = if j >= 0 { b[j as usize] } else { 0 };
        let sum = d1 + d2 + carry;
        carry = sum / 10;
        res.push(sum % 10);
        i -= 1;
        j -= 1;
    }
    res.reverse();
    res
}

pub fn sub_digits(a: &[u8], b: &[u8]) -> (Vec<u8>, bool) {
    if compare_digit_slices(a, b) < 0 {
        let (res, _) = sub_digits(b, a);
        return (res, true);
    }
    let mut i = a.len() as isize - 1;
    let mut j = b.len() as isize - 1;
    let mut borrow = 0;
    let mut res = Vec::with_capacity(a.len());
    while i >= 0 {
        let mut d1 = a[i as usize] as i8 - borrow;
        let d2 = if j >= 0 { b[j as usize] as i8 } else { 0 };
        if d1 < d2 {
            d1 += 10;
            borrow = 1;
        } else {
            borrow = 0;
        }
        res.push((d1 - d2) as u8);
        i -= 1;
        j -= 1;
    }
    res.reverse();
    while res.len() > 1 && res[0] == 0 {
        res.remove(0);
    }
    (res, false)
}

pub fn mul_digits(a: &[u8], b: &[u8]) -> Vec<u8> {
    let n = a.len();
    let m = b.len();
    let mut res = vec![0u32; n + m];

    for i in (0..n).rev() {
        for j in (0..m).rev() {
            let mul = (a[i] as u32) * (b[j] as u32);
            let sum = res[i + j + 1] + mul;
            res[i + j + 1] = sum % 10;
            res[i + j] += sum / 10;
        }
    }
    let mut result: Vec<u8> = Vec::with_capacity(n + m);
    let mut leading = true;
    for d in res {
        if leading && d == 0 {
            continue;
        }
        leading = false;
        result.push(d as u8);
    }
    if result.is_empty() {
        result.push(0);
    }
    result
}

pub fn div_digits(a: &[u8], b: &[u8]) -> Vec<u8> {
    if b.iter().all(|&x| x == 0) {
        return vec![];
    }
    let mut dividend = Vec::new();
    let mut quotient = Vec::with_capacity(a.len());
    for &digit in a {
        dividend.push(digit);
        dividend = trim_leading_zeros(dividend);
        let mut count = 0;
        while compare_digit_slices(&dividend, b) >= 0 {
            dividend = sub_digits(&dividend, b).0;
            count += 1;
        }
        quotient.push(count);
    }
    trim_leading_zeros(quotient)
}
