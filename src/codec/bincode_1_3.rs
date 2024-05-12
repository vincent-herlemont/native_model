//! ⚠️ [`Read the docs before using`](crate::bincode_1_3::Bincode#warning) -
//! Annotate your type with `native_model::bincode_1_3::Bincode` to use the
//! bincode 1.3 crate for serializing & deserializing.

/// Used to specify the [bincode](https://crates.io/crates/bincode/1.3.3) `1.3`
/// crate for serialization & deserialization.
///
/// # Warning
///
/// `bincode` [does not implement](https://github.com/bincode-org/bincode/issues/548)
/// all [serde](https://crates.io/crates/serde) features. Errors may be
/// encountered when using this with some types.
///
/// # Basic usage
///
/// Use the [`with`](crate::native_model) attribute on your type to instruct
/// `native_model` to use `Bincode` for serialization & deserialization.
///
/// Example:
///
/// ```rust
/// #[native_model(id = 1, version = 1, with = native_model::bincode_1_3::Bincode)]
/// struct MyStruct {
///     my_string: String
/// }
/// ```

#[doc(cfg(all(feature = "serde", feature = "bincode_1_3")))]
#[derive(Default)]
pub struct Bincode;

#[cfg(all(feature = "serde", feature = "bincode_1_3"))]
impl<T: serde::Serialize> super::Encode<T> for Bincode {
    type Error = bincode_1_3::Error;
    /// Serializes a type into bytes using the `bincode` `1.3` crate.
    fn encode(obj: &T) -> Result<Vec<u8>, Self::Error> {
        bincode_1_3::serialize(obj)
    }
}

#[cfg(all(feature = "serde", feature = "bincode_1_3"))]
impl<T: for<'de> serde::Deserialize<'de>> super::Decode<T> for Bincode {
    type Error = bincode_1_3::Error;
    /// Deserializes a type from bytes using the `bincode` `1.3` crate.
    fn decode(data: Vec<u8>) -> Result<T, Self::Error> {
        bincode_1_3::deserialize(&data[..])
    }
}
