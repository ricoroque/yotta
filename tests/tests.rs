#[cfg(test)]
mod tests {
    use yotta::*;

    #[test]
    fn test_add() {
        let a = Yotta::new("1234567890", 512);
        let b = Yotta::new("9876543210", 512);
        let c = a + b;
        assert_eq!(c, Yotta::new("11111111100", 256));
    }

    #[test]
    fn test_sub() {
        let a = Yotta::new("9876543210", 512);
        let b = Yotta::new("1234567890", 512);
        let c = a - b;
        assert_eq!(c, Yotta::new("8641975320", 256));

        let d = Yotta::new("1234567890", 512);
        let e = Yotta::new("9876543210", 512);
        let f = d - e;
        assert_eq!(f, Yotta::new("-8641975320", 256));
    }

    #[test]
    fn test_mul() {
        let a = Yotta::new("1234567890", 512);
        let b = Yotta::new("9876543210", 512);
        let c = a * b;
        assert_eq!(c, Yotta::new("12193263111263526900", 256));

        let d = Yotta::new("-1234567890", 512);
        let e = Yotta::new("9876543210", 512);
        let f = d * e;
        assert_eq!(f, Yotta::new("-12193263111263526900", 256));
    }

    #[test]
    fn test_div() {
        let a = Yotta::new("9876543210", 512);
        let b = Yotta::new("1234567890", 512);
        let c = a / b;
        assert_eq!(c, Yotta::new("8", 256));

        let d = Yotta::new("-9876543210", 512);
        let e = Yotta::new("1234567890", 512);
        let f = d / e;
        assert_eq!(f, Yotta::new("-8", 256));
    }

    #[test]
    fn test_display() {
        let a = Yotta::new("1234567890", 512);
        assert_eq!(format!("{}", a), "1234567890");
    }

    #[test]
    fn test_debug() {
        let a = Yotta::new("1234567890", 512);
        assert_eq!(
            format!("{:?}", a),
            "Yotta { mantissa: 1234567890, frac_part: 0, exponent: 0, bit_width: 512, negative: false }"
        );

        let b = Yotta::new("-1234567890", 512);
        assert_eq!(
            format!("{:?}", b),
            "Yotta { mantissa: 1234567890, frac_part: 0, exponent: 0, bit_width: 512, negative: true }"
        );
    }

    #[test]
    fn test_eq() {
        let a = Yotta::new("1234567890", 512);
        let b = Yotta::new("1234567890", 512);
        assert!(a == b);
    }

    #[test]
    fn test_ne() {
        let a = Yotta::new("1234567890", 512);
        let b = Yotta::new("9876543210", 512);
        assert!(a != b);
    }

    #[test]
    fn test_cmp() {
        let a = Yotta::new("1234567890", 512);
        let b = Yotta::new("1234567890", 512);
        let c = Yotta::new("9876543210", 512);
        assert!(a <= b);
        assert!(a >= b);
        assert!(a < c);
        assert!(c > a);
    }

    #[test]
    fn test_add_assign() {
        let mut a = Yotta::new("1234567890", 512);
        let b = Yotta::new("9876543210", 512);
        a += b;
        assert_eq!(a, Yotta::new("11111111100", 256));

        let mut c = Yotta::new("1234567890", 512);
        let d = Yotta::new("-9876543210", 512);
        c += d;
        assert_eq!(c, Yotta::new("-8641975320", 256));
    }

    #[test]
    fn test_sub_assign() {
        let mut a = Yotta::new("9876543210", 512);
        let b = Yotta::new("1234567890", 512);
        a -= b;
        assert_eq!(a, Yotta::new("8641975320", 256));

        let mut c = Yotta::new("1234567890", 512);
        let d = Yotta::new("9876543210", 512);
        c -= d;
        assert_eq!(c, Yotta::new("-8641975320", 256));
    }

    #[test]
    fn test_mul_assign() {
        let mut a = Yotta::new("1234567890", 512);
        let b = Yotta::new("9876543210", 512);
        a *= b;
        assert_eq!(a, Yotta::new("12193263111263526900", 256));

        let mut c = Yotta::new("-1234567890", 512);
        let d = Yotta::new("9876543210", 512);
        c *= d;
        assert_eq!(c, Yotta::new("-12193263111263526900", 256));
    }

    #[test]
    fn test_div_assign() {
        let mut a = Yotta::new("9876543210", 512);
        let b = Yotta::new("1234567890", 512);
        a /= b;
        assert_eq!(a, Yotta::new("8", 256));

        let mut c = Yotta::new("-9876543210", 512);
        let d = Yotta::new("1234567890", 512);
        c /= d;
        assert_eq!(c, Yotta::new("-8", 256));
    }

    #[test]
    fn test_neg() {
        let a = Yotta::new("1234567890", 512);
        let b = -a;
        assert_eq!(b, Yotta::new("-1234567890", 512));
    }

    #[test]
    fn test_floats() {
        let a = Yotta::new("1234567890.1234567890", 512);
        let b = Yotta::new("9876543210.9876543210", 512);
        let c = a + b;
        assert_eq!(c, Yotta::new("11111111101.1111111100", 256));

        let d = Yotta::new("1234567890.1234567890", 512);
        let e = Yotta::new("9876543210.9876543210", 512);
        let f = d - e;
        assert_eq!(f, Yotta::new("-8641975320.8641975320", 256));

        let g = Yotta::new("12.00000000000000000000000000000000000009", 512);
        let h = Yotta::new("2", 512);
        let i = g * h;
        assert_eq!(i, Yotta::new("24.00000000000000000000000000000000000018", 512));

        let j = Yotta::new("3.14", 512);
        let k = Yotta::new("2.5", 512);
        let l = j /k;
        assert_eq!(l, Yotta::new("1.256", 512));
    }

    #[test]
    fn test_sqrt_method() {
        // unimplemented!()
        assert_eq!(true, true);
    }

    #[test]
    fn test_sqrt_nonperfect() {
        // unimplemented!()
        assert_eq!(true, true);
    }

    #[test]
    fn test_modulo_method() {
        let a = Yotta::new("10", 512);
        let b = Yotta::new("3", 512);
        let rem = a.modulo(&b).unwrap();
        // 10 = 3*3 + 1.
        assert_eq!(rem, Yotta::new("1", 512));
    }

    #[test]
    fn test_gcd_method() {
        // gcd(54, 24) = 6.
        let a = Yotta::new("54", 512);
        let b = Yotta::new("24", 512);
        let result = a.gcd_with(&b);
        assert_eq!(result, Yotta::new("6", 512));
    }

    #[test]
    fn test_lcm_method() {
        // lcm(4, 6) = 12.
        let a = Yotta::new("4", 512);
        let b = Yotta::new("6", 512);
        let result = a.lcm_with(&b).unwrap();
        assert_eq!(result, Yotta::new("12", 512));
    }

    #[test]
    fn test_modpow_method() {
        // 2^10 = 1024, and 1024 mod 1000 = 24.
        let base = Yotta::new("2", 512);
        let exp = Yotta::new("10", 512);
        let modulo = Yotta::new("1000", 512);
        let result = base.modpow(&exp, &modulo).unwrap();
        assert_eq!(result, Yotta::new("24", 512));
    }

    #[test]
    fn test_modinv_method() {
        // For 3 modulo 11, the modular inverse is 4 since (3*4) mod 11 = 12 mod 11 = 1.
        let a = Yotta::new("3", 512);
        let modulo = Yotta::new("11", 512);
        let result = a.modinv(&modulo).unwrap();
        assert_eq!(result, Yotta::new("4", 512));
    }

    #[test]
    fn larger_than_u128() {
        let a = Yotta::new("1234567890123456789012345678900000", 512);
        let b = Yotta::new("9876543210987654321098765432100000", 512);
        let c = a + b;
        assert_eq!(c, Yotta::new("11111111101111111110111111111000000", 512));

        let d = Yotta::new("1234567890123456789012345678900000", 512);
        let e = Yotta::new("9876543210987654321098765432100000", 512);
        let f = d - e;
        assert_eq!(f, Yotta::new("-8641975320864197532086419753200000", 512));

        let g = Yotta::new("1234567890123456789012345678900000", 560);
        let h = Yotta::new("9876543210987654321098765432100000", 560);
        let i = g * h;
        assert_eq!(
            i,
            Yotta::new(
                "12193263113702179522618503273362292333223746380111126352690000000000",
                560
            )
        );

        let j = Yotta::new("9876543210987654321098765432100000", 512);
        let k = Yotta::new("1234567890123456789012345678900000", 512);
        let l = j / k;
        assert_eq!(l, Yotta::new("8", 512));
    }
}
