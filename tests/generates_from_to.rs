use try_from_derive::*;

#[derive(PartialEq, Debug)]
enum Wrap {
    Float(f32),
    Int(i32),
}

#[test]
fn cast_to() {
    assert_eq!(Wrap::from(1.0 as f32), Wrap::Float(1.0));
    assert_eq!(Wrap::from(1 as i32), Wrap::Int(1));
}