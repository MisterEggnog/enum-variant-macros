use enum_variant_macros::*;
#[derive(FromVariants)]
enum Variants {
	Int(i32),
	Float(f32),
	None,
}
fn main() {
}
