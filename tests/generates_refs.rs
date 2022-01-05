use strum_macros::IntoStaticStr;
use try_from_derive::*;

#[allow(dead_code)]
#[derive(IntoStaticStr, TryFromVariants)]
enum Variants {
    Int(i32),
    Float(f32),
}

impl<'a> TryFrom<&'a Variants> for &'a f32 {
    type Error = ::try_from_derive::VariantCastError;

    fn try_from(value: &'a Variants) -> ::std::result::Result<Self, Self::Error> {
        match value {
            Variants::Float(n) => Ok(&n),
            _ => Err(::try_from_derive::VariantCastError {
                enum_type: stringify!(Variants),
                exp_type: stringify!(Float),
                variant_name: value.into(),
            }),
        }
    }
}

#[test]
fn can_access_reference() {
    let var = Variants::Float(7.11);
    let var_ref = &var;

    let try_ref: &f32 = TryFrom::try_from(var_ref).expect("This should return a f32 reference");
    assert_eq!(try_ref, &7.11);
    //assert_eq!(<&i32>::try_from(var_ref).is_err());
}
