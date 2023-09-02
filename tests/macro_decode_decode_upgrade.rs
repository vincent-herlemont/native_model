use bincode::{config, Decode, Encode};
use native_model::Model;
use native_model_macro::native_model;

fn native_model_encode_body<T: Encode>(obj: &T) -> Result<Vec<u8>, bincode::error::EncodeError> {
    bincode::encode_to_vec(obj, config::standard())
}

fn native_model_decode_body<T: Decode>(data: Vec<u8>) -> Result<T, bincode::error::DecodeError> {
    bincode::decode_from_slice(&data, config::standard()).map(|(result, _)| result)
}

#[derive(Debug, Encode, Decode, PartialEq)]
#[native_model(id = 1, version = 1)]
struct Foo1 {
    x: i32,
}

#[derive(Debug, Encode, Decode, PartialEq)]
#[native_model(id = 1, version = 2, from = Foo1)]
struct Foo2 {
    x: String,
}

impl From<Foo1> for Foo2 {
    fn from(foo1: Foo1) -> Self {
        Foo2 {
            x: foo1.x.to_string(),
        }
    }
}

impl From<Foo2> for Foo1 {
    fn from(foo2: Foo2) -> Self {
        Foo1 {
            x: foo2.x.parse::<i32>().unwrap(),
        }
    }
}

#[derive(Debug, Encode, Decode, PartialEq)]
#[native_model(id = 1, version = 3, from = Foo2)]
enum Foo3 {
    X(i32),
}

impl From<Foo2> for Foo3 {
    fn from(foo2: Foo2) -> Self {
        Foo3::X(foo2.x.parse::<i32>().unwrap())
    }
}

impl From<Foo3> for Foo2 {
    fn from(foo3: Foo3) -> Self {
        match foo3 {
            Foo3::X(x) => Foo2 { x: x.to_string() },
        }
    }
}

#[test]
fn test_decode_foo1_to_foo2() {
    let foo1 = Foo1 { x: 100 };
    let foo1_encoded = foo1.native_model_encode_body().unwrap();
    let foo2_decoded = Foo2::native_model_decode_upgrade_body(foo1_encoded, 1).unwrap();
    assert_eq!(foo1.x.to_string(), foo2_decoded.x);
}

#[test]
fn test_decode_foo2_to_foo3() {
    let foo2 = Foo2 {
        x: "100".to_string(),
    };
    let foo2_encoded = foo2.native_model_encode_body().unwrap();
    let foo3_decoded = Foo3::native_model_decode_upgrade_body(foo2_encoded, 2).unwrap();
    assert_eq!(Foo3::X(100), foo3_decoded);
}

#[test]
fn test_decode_foo1_to_foo3() {
    let foo1 = Foo1 { x: 100 };
    let foo1_encoded = foo1.native_model_encode_body().unwrap();
    let foo3_decoded = Foo3::native_model_decode_upgrade_body(foo1_encoded, 1).unwrap();
    assert_eq!(Foo3::X(100), foo3_decoded);
}

#[test]
fn test_decode_foo1_to_foo1() {
    let foo1 = Foo1 { x: 100 };
    let foo1_encoded = foo1.native_model_encode_body().unwrap();
    let foo1_decoded = Foo1::native_model_decode_upgrade_body(foo1_encoded, 1).unwrap();
    assert_eq!(foo1, foo1_decoded);
}

#[test]
fn test_decode_foo2_to_foo2() {
    let foo2 = Foo2 {
        x: "100".to_string(),
    };
    let foo2_encoded = foo2.native_model_encode_body().unwrap();
    let foo2_decoded = Foo2::native_model_decode_upgrade_body(foo2_encoded, 2).unwrap();
    assert_eq!(foo2, foo2_decoded);
}

#[test]
fn test_decode_foo3_to_foo3() {
    let foo3 = Foo3::X(100);
    let foo3_encoded = foo3.native_model_encode_body().unwrap();
    let foo3_decoded = Foo3::native_model_decode_upgrade_body(foo3_encoded, 3).unwrap();
    assert_eq!(foo3, foo3_decoded);
}

#[test]
fn test_should_fail_decode_foo3_to_foo2() {
    let foo3 = Foo3::X(100);
    let foo3_encoded = foo3.native_model_encode_body().unwrap();
    let foo3_decoded = Foo2::native_model_decode_upgrade_body(foo3_encoded, 3);
    assert!(foo3_decoded.is_err());
    assert!(matches!(
        foo3_decoded.unwrap_err(),
        native_model::Error::UpgradeNotSupported { from: 3, to: 2 }
    ));
}

#[test]
fn test_should_fail_decode_foo3_to_foo1() {
    let foo3 = Foo3::X(100);
    let foo3_encoded = foo3.native_model_encode_body().unwrap();
    let foo3_decoded = Foo1::native_model_decode_upgrade_body(foo3_encoded, 3);
    assert!(foo3_decoded.is_err());
    assert!(matches!(
        foo3_decoded.unwrap_err(),
        native_model::Error::UpgradeNotSupported { from: 3, to: 1 }
    ));
}

#[test]
fn test_should_fail_decode_foo2_to_foo1() {
    let foo2 = Foo2 {
        x: "100".to_string(),
    };
    let foo2_encoded = foo2.native_model_encode_body().unwrap();
    let foo2_decoded = Foo1::native_model_decode_upgrade_body(foo2_encoded, 2);
    assert!(foo2_decoded.is_err());
    assert!(matches!(
        foo2_decoded.unwrap_err(),
        native_model::Error::UpgradeNotSupported { from: 2, to: 1 }
    ));
}
