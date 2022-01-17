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
let variant = Variants::Float(12);
let float: f32 = variant.try_from()?;
```
> Note that derivation of this type also requires that the `impl From<Variant> for &'static str` is also implemented.

FromVariants does not have any such restrictions.

## Warnings
There remains a lack of proper error handling, while work has been done in that direction their remains severe edge cases.
Most prominently there is currently no checks for having more than one variant of a type, until the compiler attempts to parse the results.
