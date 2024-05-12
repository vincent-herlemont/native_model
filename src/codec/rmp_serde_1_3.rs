//! [`Annotate your type`](crate::native_model) with
//! `native_model::rmp_serde_1_3::RmpSerde` to use the rmp-serde 1.3 crate for
//! serializing & deserializing.

/// Used to specify the [rmp-serde](https://crates.io/crates/rmp-serde/1.3.0)
/// `1.3` crate for serialization & deserialization.
///
/// # Basic usage
///
/// Use the [`with`](crate::native_model) attribute on your type to instruct
/// `native_model` to use `RmpSerde` for serialization & deserialization.
///
/// Example:
///
/// ```rust
/// #[native_model(id = 1, version = 1, with = native_model::rmp_serde_1_3::RmpSerde)]
/// struct MyStruct {
///     my_string: String
/// }
/// ```

#[doc(cfg(all(feature = "serde", feature = "rmp_serde_1_3")))]
pub struct RmpSerde;

#[cfg(all(feature = "serde", feature = "rmp_serde_1_3"))]
impl<T: serde::Serialize> crate::Encode<T> for RmpSerde {
    type Error = rmp_serde_1_3::encode::Error;
    /// Serializes a type into bytes using the `rmp-serde` `1.3` crate.
    fn encode(obj: &T) -> Result<Vec<u8>, Self::Error> {
        rmp_serde_1_3::encode::to_vec(obj)
    }
}

#[cfg(all(feature = "serde", feature = "rmp_serde_1_3"))]
impl<T: for<'de> serde::Deserialize<'de>> crate::Decode<T> for RmpSerde {
    type Error = rmp_serde_1_3::decode::Error;
    /// Deserializes a type from bytes using the `rmp-serde` `1.3` crate.
    fn decode(data: Vec<u8>) -> Result<T, Self::Error> {
        rmp_serde_1_3::decode::from_slice(&data)
    }
}