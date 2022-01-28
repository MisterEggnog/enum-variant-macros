use enum_variant_macros::*;
use strum_macros::IntoStaticStr;
use std::error::Error;

#[derive(PartialEq, Debug, IntoStaticStr, FromVariants, TryFromVariants)]
enum Variants {
	Int(i32),
	Point(f32),
}

#[test]
fn tuple_from_try_from() -> Result<(), Box<dyn Error>> {
	let source = Variants::Point(12.0);
	let expected = (12.0);
	let result: f32 = TryFrom::try_from(source)?;
	assert_eq!(expected, result);

	let source = Variants::Int(122);
	let result = f32::try_from(source);
	assert!(result.is_err());

	Ok(())
}

#[test]
fn tuple_to_variant() {
	let source = (12.0);
	let expected = Variants::Point(12.0);
	let result = Variants::from(source);
	assert_eq!(expected, result);
}
