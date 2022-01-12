use enum_variant_macros::*;
#[derive(FromVariants)]
union NotEnum {
	a: u32,
	b: f32,
}
fn main() {
}
