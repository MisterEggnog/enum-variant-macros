use enum_variant_macros::*;
#[derive(TryFromVariants)]
enum Variants {
	Shell(i32),
	Wite(f32),
	Ell(i32),
}
fn main() {
}
