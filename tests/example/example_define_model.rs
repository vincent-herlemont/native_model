use bincode::{config, Decode, Encode};
use native_model_macro::native_model;

#[allow(dead_code)]
fn native_model_encode_body<T: Encode>(obj: &T) -> Result<Vec<u8>, bincode::error::EncodeError> {
    bincode::encode_to_vec(obj, config::standard())
}

#[allow(dead_code)]
fn native_model_decode_body<T: Decode>(data: Vec<u8>) -> Result<T, bincode::error::DecodeError> {
    bincode::decode_from_slice(&data, config::standard()).map(|(result, _)| result)
}

#[derive(Encode, Decode, PartialEq, Debug)]
#[native_model(id = 1, version = 1)]
struct DotV1(u32, u32);

#[derive(Encode, Decode, PartialEq, Debug)]
#[native_model(id = 1, version = 2, from = DotV1)]
struct DotV2 {
    name: String,
    x: u64,
    y: u64,
}

impl From<DotV1> for DotV2 {
    fn from(dot: DotV1) -> Self {
        DotV2 {
            name: "".to_string(),
            x: dot.0 as u64,
            y: dot.1 as u64,
        }
    }
}

impl From<DotV2> for DotV1 {
    fn from(dot: DotV2) -> Self {
        DotV1(dot.x as u32, dot.y as u32)
    }
}

#[derive(Encode, Decode, PartialEq, Debug)]
#[native_model(id = 1, version = 3, try_from = (DotV2, anyhow::Error))]
struct DotV3 {
    name: String,
    cord: Cord,
}

#[derive(Encode, Decode, PartialEq, Debug)]
struct Cord {
    x: u64,
    y: u64,
}

impl TryFrom<DotV2> for DotV3 {
    type Error = anyhow::Error;

    fn try_from(dot: DotV2) -> Result<Self, Self::Error> {
        Ok(DotV3 {
            name: dot.name,
            cord: Cord { x: dot.x, y: dot.y },
        })
    }
}

impl TryFrom<DotV3> for DotV2 {
    type Error = anyhow::Error;

    fn try_from(dot: DotV3) -> Result<Self, Self::Error> {
        Ok(DotV2 {
            name: dot.name,
            x: dot.cord.x,
            y: dot.cord.y,
        })
    }
}

#[test]
fn simple_test() {
    let dot = DotV1(1, 2);
    let bytes = native_model::encode(&dot).unwrap();

    let (dot_decoded, _) = native_model::decode::<DotV1>(bytes.clone()).unwrap();
    assert_eq!(dot, dot_decoded);

    let (dot_decoded, _) = native_model::decode::<DotV2>(bytes.clone()).unwrap();
    assert_eq!(
        DotV2 {
            name: "".to_string(),
            x: 1,
            y: 2
        },
        dot_decoded
    );

    let (dot_decoded, _) = native_model::decode::<DotV3>(bytes.clone()).unwrap();
    assert_eq!(
        DotV3 {
            name: "".to_string(),
            cord: Cord { x: 1, y: 2 }
        },
        dot_decoded
    );
}
