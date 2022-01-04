use strum_macros::IntoStaticStr;
use try_from_derive::*;

#[allow(dead_code)]
#[derive(IntoStaticStr, TryFromVariants)]
enum Variants {
	Int(i32),
	Float(f32),
}

#[test]
fn can_access_reference() {
	let var = Variants::Float(7.11);
	let var_ref = &var;

	let try_ref: &f32 = TryFrom::try_from(var_ref).expect("This should return a f32 reference");
	assert_eq!(try_ref, &7.11);
	assert_eq!(<&i32>::try_from(var_ref).is_err());
}
