//! ⚠️ [`Read the docs before using`](crate::postcard_1_0::PostCard#warning) -
//! Annotate your type with `native_model::postcard_1_0::PostCard` to
//! use the postcard 1.0 crate for serializing & deserializing.

/// Used to specify the [postcard](https://crates.io/crates/postcard/1.0.8)
/// `1.0` crate for serialization & deserialization.
///
/// # Warning
///
/// `postcard` does not implement all [serde](https://crates.io/crates/serde)
/// features. Errors may be encountered when using this with some types.
///
/// # Basic usage
///
/// Use the [`with`](crate::native_model) attribute on your type to instruct
/// `native_model` to use `PostCard` for serialization & deserialization.
///
/// Example:
///
/// ```rust
/// #[native_model(id = 1, version = 1, with = native_model::postcard_1_0::PostCard)]
/// struct MyStruct {
///     my_string: String
/// }
/// ```

#[doc(cfg(all(feature = "serde", feature = "postcard_1_0")))]
pub struct PostCard;

#[cfg(all(feature = "serde", feature = "postcard_1_0"))]
impl<T: serde::Serialize> super::Encode<T> for PostCard {
    type Error = postcard_1_0::Error;
    /// Serializes a type into bytes using the `postcard` `1.0` crate.
    fn encode(obj: &T) -> Result<Vec<u8>, Self::Error> {
        postcard_1_0::to_allocvec(obj)
    }
}

#[cfg(all(feature = "serde", feature = "postcard_1_0"))]
impl<T: for<'de> serde::Deserialize<'de>> super::Decode<T> for PostCard {
    type Error = postcard_1_0::Error;
    /// Deserializes a type from bytes using the `postcard` `1.0` crate.
    fn decode(data: Vec<u8>) -> Result<T, Self::Error> {
        postcard_1_0::from_bytes(&data)
    }
}