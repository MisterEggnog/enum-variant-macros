# Enum Variant Macros
Macro library for use in generating From & TryFrom  implementations for enums composed of unnamed single members.
In other terms, of the following format.
```rust
pub enum Variants {
    Integer(i32),
    Float(f32),
    Bool(bool),
    Byte(u8),
}
```
This library has 2 macros, TryFromVariants & FromVariants.

TryFromVariants implements TryFrom for each of the variant types, permitting code such as,
```rust
use enum_variant_macros::*;
use std::error::Error;
use strum_macros::IntoStaticStr;

#[derive(IntoStaticStr, TryFromVariants)]
enum Variants {
    Integer(i32),
    Float(f32),
}

fn main() -> Result<(), Box<dyn Error>> {
    let variant = Variants::Float(12.0);
    let float: f32 = TryFrom::try_from(variant)?;
    Ok(())
}
```
Note: Derivation of this type also requires that `impl From<Variant> for &'static str` is implemented.

FromVariants is relatively simple, it just generates From for each wrapped type.
```rust
use enum_variant_macros::*;

#[derive(Debug, PartialEq, FromVariants)]
enum Variants {
    Integer(i32),
    Float(f32),
}

let variant = Variants::from(12);
assert_eq!(Variants::Integer(12), variant);
```

## Note
If this library is not to your preference, may I recomend [enum dispatch], a more expansive library that also provides from & try_into for enums.

## License
Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT license](LICENSE-MIT) at your option.

[enum dispatch]: https://lib.rs/crates/enum_dispatch
