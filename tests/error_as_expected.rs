use strum_macros::AsRefStr;
use try_from_derive::*;

mod common;
use common::StackException;

#[allow(dead_code)]
#[derive(AsRefStr, TryFromVariants)]
enum GenEnum {
    Word(u32),
    NotWord(i8),
}

#[test]
fn error_actual_type_full_enum_name() {
    let a = GenEnum::NotWord(0);
    let b = u32::try_from(a);
    let b = b.err().expect("Succeeded in casting incorrect enum");
    match b {
        StackException::InvalidStackValue {
            exp_type: a,
            act_type: b,
        } => {
            assert_eq!("GenEnum::Word", a);
            assert_eq!("GenEnum::NotWord", b);
        }
        _ => panic!("Cast failed but for incorrect exception: {}", b),
    }
}
