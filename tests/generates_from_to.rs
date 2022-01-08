use try_from_derive_proc::FromVariants;

#[derive(PartialEq, Debug, FromVariants)]
enum Wrap {
    Float(f32),
    Int(i32),
}

#[test]
fn cast_to() {
    assert_eq!(Wrap::from(1.0_f32), Wrap::Float(1.0));
    assert_eq!(Wrap::from(1_i32), Wrap::Int(1));
}
