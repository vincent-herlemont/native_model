use bincode;
use bincode::{Decode, Encode};
use native_model::native_model;

fn native_model_encode_body<T: bincode::Encode>(
    model: &T,
) -> Result<Vec<u8>, bincode::error::EncodeError> {
    {
        bincode::encode_to_vec(model, bincode::config::standard())
    }
}

fn native_model_decode_body<T: bincode::Decode>(
    data: Vec<u8>,
) -> Result<T, bincode::error::DecodeError> {
    {
        bincode::decode_from_slice(&data, bincode::config::standard()).map(|(result, _)| result)
    }
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

#[test]
fn run_example() {
    // Application 1
    let dot = DotV1(1, 2);
    let bytes = native_model::encode(&dot).unwrap();

    // Application 1 sends bytes to Application 2.

    // Application 2
    let (mut dot, source_version) = native_model::decode::<DotV2>(bytes).unwrap();
    // Use the struct DataV2 which has more fields and a different structure.
    dot.name = "Dot".to_string();
    dot.x = 5;
    // Encode the dot with the application 1 version in order to be compatible with it.
    let bytes = native_model::encode_downgrade(dot, source_version).unwrap();

    // Application 2 sends bytes to Application 1.

    // Application 1
    let (dot, _) = native_model::decode::<DotV1>(bytes).unwrap();
    assert_eq!(dot, DotV1(5, 2));
}
