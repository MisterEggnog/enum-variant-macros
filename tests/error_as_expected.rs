use strum_macros::IntoStaticStr;
use try_from_derive::*;

#[allow(dead_code)]
#[derive(IntoStaticStr, TryFromVariants)]
enum GenEnum {
    Word(u32),
    NotWord(i8),
}

#[test]
fn error_as_expected() {
    let a = GenEnum::NotWord(0);
    let b = u32::try_from(a);
    let b = b.err().expect("Succeeded in casting incorrect enum");
    assert_eq!("GenEnum", b.enum_type);
    assert_eq!("u32", b.exp_type);
    assert_eq!("NotWord", b.variant_name);
}
