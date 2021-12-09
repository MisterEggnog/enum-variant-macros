use strum_macros::AsRefStr;
use try_from_derive_proc::*;

mod common;
use common::VariantCastError;

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
    assert_eq!("GenEnum::Word", b.exp_type);
    assert_eq!("GenEnum::NotWord", b.act_type);
}
