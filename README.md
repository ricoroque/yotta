# Yotta
Yotta is a pure Rust arbitrary-precision arithmetic library. It is designed to be fast and easy to use. Yotta is still in development, so it is not recommended for production use.

Yotta is named after `Yottabyte`, the largest unit of digital information storage. This is because Yotta can handle numbers of any size.

## Features
- [x] Dynamic precision arithmetic
- [x] Traits for using operators
- [x] Floating point arithmetic
- [x] Easy to use
- [x] `no_std`

## Usage
First, add Yotta to your `Cargo.toml`:
```bash
cargo add yotta
```

Then, you can use Yotta in your project:
```rust
use yotta::Yotta;

fn main() {
    let num1 = Yotta::new("123", 32);
    let num2 = Yotta::new("456", 32);

    let sum = num1.clone() + num2.clone();
    let diff = num2.clone() - num1.clone();

    assert_eq!(sum, Yotta::new("579", 32));
    assert_eq!(diff, Yotta::new("333", 32));
}
```

## License
Yotta is licensed under the [Protected Work License (PWL) v1.0](LICENSE.md). This license is designed to protect open source software from being paywalled without restricting commercial use. It also implements the MIT License.
