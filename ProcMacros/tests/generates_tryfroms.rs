use strum_macros::AsRefStr;
use try_from_derive_proc::*;

mod common;
use common::VariantCastError;

#[allow(dead_code)]
#[derive(AsRefStr, TryFromVariants)]
enum Pizza {
    Spaghetti(bool),
    Terry(i32),
}

#[test]
fn tryfrom_works_fails_for_pizza_spaghetti() {
    let source = Pizza::Spaghetti(false);
    let output =
        bool::try_from(source).expect("Casting from Pizza::Spaghetti to bool should succeed");
    assert_eq!(false, output);

    let source = Pizza::Terry(12345);
    let output = bool::try_from(source);
    assert!(output.is_err());
}

#[test]
fn tryfrom_works_fails_for_pizza_terry() {
    let source = Pizza::Terry(12345);
    let output = i32::try_from(source).expect("Casting from Pizza::Terry to i32 should succeed");
    assert_eq!(12345, output);

    let source = Pizza::Spaghetti(false);
    let output = i32::try_from(source);
    assert!(output.is_err());
}
