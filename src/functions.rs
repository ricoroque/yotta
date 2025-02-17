use crate::{Yotta, YottaError};

pub fn sqrt(_: &Yotta) -> Result<Yotta, YottaError> {
    unimplemented!()
}

pub fn modulo_op(a: &Yotta, b: &Yotta) -> Result<Yotta, YottaError> {
    let div = a.div_impl(b)?;
    let prod = div.mul_impl(b);
    let rem = a.sub_impl(&prod);
    Ok(rem)
}

pub fn gcd(a: &Yotta, b: &Yotta) -> Yotta {
    let zero = Yotta::new("0", a.bit_width);
    if *b == zero {
        return a.clone();
    }
    let rem = modulo_op(a, b).unwrap();
    gcd(b, &rem)
}

pub fn lcm(a: &Yotta, b: &Yotta) -> Result<Yotta, YottaError> {
    let prod = a.mul_impl(b);
    let g = gcd(a, b);
    prod.div_impl(&g)
}

pub fn modpow(base: &Yotta, exponent: &Yotta, modulo: &Yotta) -> Result<Yotta, YottaError> {
    let zero = Yotta::new("0", base.bit_width);
    let one = Yotta::new("1", base.bit_width);
    let two = Yotta::new("2", base.bit_width);
    let mut result = one.clone();
    let mut exp = exponent.clone();
    let mut base_mod = modulo_op(base, modulo)?;
    while exp > zero {
        if modulo_op(&exp, &two)? == one {
            result = modulo_op(&result.mul_impl(&base_mod), modulo)?;
        }
        exp = exp.div_impl(&two).unwrap();
        base_mod = modulo_op(&base_mod.mul_impl(&base_mod), modulo)?;
    }
    Ok(result)
}

fn extended_gcd(a: Yotta, b: Yotta) -> (Yotta, Yotta, Yotta) {
    let zero = Yotta::new("0", a.bit_width);
    if b == zero {
        return (a.clone(), Yotta::new("1", a.bit_width), Yotta::new("0", a.bit_width));
    }
    let quotient = a.div_impl(&b).unwrap();
    let remainder = modulo_op(&a, &b).unwrap();
    let (g, x1, y1) = extended_gcd(b, remainder);
    let x = y1.clone();
    let y = x1.sub_impl(&quotient.mul_impl(&y1));
    (g, x, y)
}

pub fn modinv(a: &Yotta, modulo: &Yotta) -> Result<Yotta, YottaError> {
    let (g, x, _y) = extended_gcd(a.clone(), modulo.clone());
    let one = Yotta::new("1", a.bit_width);
    if g != one {
        return Err(YottaError::InverseDoesNotExist);
    }
    modulo_op(&x, modulo)
}
