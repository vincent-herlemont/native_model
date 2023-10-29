use bincode::{config, Decode, Encode};
use native_model::native_model;

pub struct Bincode;

impl<T: bincode::Encode> native_model::Encode<T> for Bincode {
    type Error = bincode::error::EncodeError;
    fn encode(obj: &T) -> Result<Vec<u8>, bincode::error::EncodeError> {
        bincode::encode_to_vec(obj, config::standard())
    }
}

impl<T: bincode::Decode> native_model::Decode<T> for Bincode {
    type Error = bincode::error::DecodeError;
    fn decode(data: Vec<u8>) -> Result<T, bincode::error::DecodeError> {
        bincode::decode_from_slice(&data, config::standard()).map(|(result, _)| result)
    }
}
#[derive(Debug, Encode, Decode, PartialEq)]
#[native_model(id = 1, version = 1, with = Bincode)]
struct Foo1 {
    x: i32,
}

#[derive(Debug, Encode, Decode, PartialEq)]
#[native_model(id = 1, version = 2, with = Bincode, try_from = (Foo1, anyhow::Error))]
struct Foo2 {
    x: i32,
}

impl TryFrom<Foo1> for Foo2 {
    type Error = anyhow::Error;

    fn try_from(foo1: Foo1) -> Result<Self, Self::Error> {
        if foo1.x > 10 {
            return Err(anyhow::anyhow!("x > 10"));
        }

        Ok(Foo2 { x: foo1.x })
    }
}

impl TryFrom<Foo2> for Foo1 {
    type Error = anyhow::Error;

    fn try_from(foo2: Foo2) -> Result<Self, Self::Error> {
        if foo2.x > 10 {
            return Err(anyhow::anyhow!("x > 10"));
        }

        Ok(Foo1 { x: foo2.x })
    }
}

#[test]
fn test_foo1_to_foo1() {
    let foo1 = Foo1 { x: 1 };
    let foo1_packed = native_model::encode(&foo1).unwrap();
    let (foo1_decoded, _) = native_model::decode::<Foo1>(foo1_packed.clone()).unwrap();
    assert_eq!(foo1, foo1_decoded);
}

#[test]
fn test_foo1_to_foo2() {
    let foo1 = Foo1 { x: 1 };
    let foo1_packed = native_model::encode(&foo1).unwrap();
    let (foo2_decoded, _) = native_model::decode::<Foo2>(foo1_packed.clone()).unwrap();
    assert_eq!(Foo2 { x: 1 }, foo2_decoded);
}

#[test]
fn test_foo1_to_foo2_error() {
    let foo1 = Foo1 { x: 1000 };
    let foo1_packed = native_model::encode(&foo1).unwrap();
    let foo2_decoded = native_model::decode::<Foo2>(foo1_packed.clone());
    assert!(foo2_decoded.is_err());
    assert!(matches!(
        foo2_decoded.unwrap_err(),
        native_model::Error::UpgradeError(_)
    ));
}
