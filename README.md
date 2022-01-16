# Enum Variant Macros
Macro library for use in generating From & TryFrom for enums composed of unnamed single members.

It is probably easier to just give an example, out of a lack of knowing a more accurate word.
```rust
pub enum Variants {
	Integer(i32),
	Float(f32),
	Bool(bool),
	Strang(String),
}
```