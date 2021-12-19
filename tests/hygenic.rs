use ::std::mem::size_of;
use strum_macros::IntoStaticStr;
use try_from_derive::TryFromVariants;

#[allow(dead_code)]
#[derive(IntoStaticStr, TryFromVariants)]
enum Either {
    Left(u16),
    Right(f32),
}

#[allow(dead_code)]
pub mod std {
    pub mod result {
        pub struct Result;
    }
}

// Try to shadow std Result
use self::std::result::Result;

#[test]
fn macro_does_not_need_error_use_statement() {
    assert_eq!(size_of::<Result>(), 0, "Result has not been shadowed");

    let v = u16::try_from(Either::Right(0.0));
    assert!(v.is_err());
}
