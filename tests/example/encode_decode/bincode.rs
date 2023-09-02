use bincode;
use bincode::{Decode, Encode};

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

use native_model_macro::native_model;

#[derive(Encode, Decode, PartialEq, Debug)]
#[native_model(id = 1, version = 1)]
struct DotV1(u32, u32);

#[test]
fn test_bincode_encode_decode() {
    // Application 1
    let dot = DotV1(1, 2);
    let bytes = native_model::encode(&dot).unwrap();
    // Application 1
    let (dot, _) = native_model::decode::<DotV1>(bytes).unwrap();
    assert_eq!(dot, DotV1(1, 2));
}
