use enum_variant_macros::*;
#[derive(FromVariants)]
enum Variants {
	Int(i32),
	Float(f32),
	Point(i32, i32),
}
fn main() {
}
