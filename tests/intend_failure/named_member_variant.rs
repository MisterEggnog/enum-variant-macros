use enum_variant_macros::*;
#[derive(FromVariants)]
enum Variants {
	Int(i32),
	Float(f32),
	Point{ x: i32, y: i32 },
}
fn main() {
}
