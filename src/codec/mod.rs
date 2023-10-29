#[cfg(all(feature = "serde", feature = "bincode_1_3"))]
pub mod bincode_1_3;
#[cfg(all(feature = "serde", feature = "bincode_2_rc"))]
pub mod bincode_2_rc;
#[cfg(all(feature = "serde", feature = "postcard_1_0"))]
pub mod postcard_1_0;

/// Encode trait for your own encoding method.
///
/// Example:
/// ```rust
/// pub struct Bincode;
///
/// impl<T: bincode::Encode> native_model::Encode<T> for Bincode {
///     type Error = bincode::error::EncodeError;
///     fn encode(obj: &T) -> Result<Vec<u8>, bincode::error::EncodeError> {
///         bincode::encode_to_vec(obj, bincode::config::standard())
///     }
/// }
/// ```
pub trait Encode<T> {
    type Error;
    fn encode(obj: &T) -> Result<Vec<u8>, Self::Error>;
}

/// Decode trait for your own decoding method.
///
/// Example:
/// ```rust
/// pub struct Bincode;
///
/// impl<T: bincode::Decode> native_model::Decode<T> for Bincode {
///     type Error = bincode::error::DecodeError;
///     fn decode(data: Vec<u8>) -> Result<T, bincode::error::DecodeError> {
///         bincode::decode_from_slice(&data, bincode::config::standard()).map(|(result, _)| result)
///     }
/// }
pub trait Decode<T> {
    type Error;
    fn decode(data: Vec<u8>) -> Result<T, Self::Error>;
}
