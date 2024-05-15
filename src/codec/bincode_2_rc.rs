//! [bincode 2.0.0-rc.3](https://crates.io/crates/bincode/2.0.0-rc.3) ·
//! Enable the `bincode_2_rc` feature and annotate your type with
//! `native_model::bincode_2_rc::Bincode` to have `native_db` use this crate for
//! serializing & deserializing.

/// Used to specify the
/// [bincode 2.0.0-rc.3](https://crates.io/crates/bincode/2.0.0-rc.3)
/// crate for serialization & deserialization.
///
/// # Warning
///
/// `bincode` [does not implement](https://docs.rs/bincode/2.0.0-rc.3/bincode/serde/index.html#known-issues)
/// all [serde](https://crates.io/crates/serde) features. Errors may be
/// encountered when using this with some types.
///
/// If you are encountering errors when using this codec on your types, try
/// using the `rmp_serde_1_3` codec instead.
///
/// # Basic usage
///
/// After enabling the `bincode_2_rc` feature in your `Cargo.toml`, use the
/// [`with`](crate::native_model) attribute on your type to instruct
/// `native_model` to use `Bincode` for serialization & deserialization.
///
/// Example usage:
///
<<<<<<< HEAD
/// ```rust
/// #[derive(Clone, Default, serde::Deserialize, serde::Serialize)]
=======
/// ```no_run
/// # fn main() {
/// use serde::{Deserialize, Serialize};
/// use native_model::native_model;
///
/// #[derive(Clone, Default, Deserialize, Serialize)]
>>>>>>> bded1fc (Turn off doc tests)
/// #[native_model(id = 1, version = 1, with = native_model::bincode_2_rc::Bincode)]
/// struct MyStruct {
///     my_string: String
/// }
/// ```

pub struct Bincode;

#[cfg(all(feature = "serde", feature = "bincode_2_rc"))]
impl<T: serde::Serialize> super::Encode<T> for Bincode {
    type Error = bincode_2_rc::error::EncodeError;
    /// Serializes a type into bytes using the `bincode` `2.0.0-rc.3` crate.
    fn encode(obj: &T) -> Result<Vec<u8>, Self::Error> {
        bincode_2_rc::serde::encode_to_vec(
            obj,
            bincode_2_rc::config::standard()
        )
    }
}

#[cfg(all(feature = "serde", feature = "bincode_2_rc"))]
impl<T: for<'de> serde::Deserialize<'de>> super::Decode<T> for Bincode {
    type Error = bincode_2_rc::error::DecodeError;
    /// Deserializes a type from bytes using the `bincode` `2.0.0-rc.3` crate.
    fn decode(data: Vec<u8>) -> Result<T, Self::Error> {
        Ok(bincode_2_rc::serde::decode_from_slice(
            &data,
            bincode_2_rc::config::standard()
        )?.0)
    }
}
