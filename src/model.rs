use crate::{DecodeResult, EncodeResult, Result};

pub trait Model: Sized {
    fn native_model_id() -> u32;
    fn native_model_version() -> u32;

    // --------------- Decode ---------------
    fn native_model_decode_body(data: Vec<u8>, id: u32) -> DecodeResult<Self>
    where
        Self: Sized;

    fn native_model_decode_upgrade_body(data: Vec<u8>, id: u32, version: u32) -> Result<Self>
    where
        Self: Sized;

    fn native_model_decode(data: Vec<u8>) -> Result<(Self, u32)>
    where
        Self: Sized,
    {
        let native_model = crate::Wrapper::deserialize(&data[..]).unwrap();
        let source_id = native_model.get_id();
        let source_version = native_model.get_version();
        let result = Self::native_model_decode_upgrade_body(
            native_model.value().to_vec(),
            source_id,
            source_version,
        )?;
        Ok((result, source_version))
    }

    // --------------- Encode ---------------

    fn native_model_encode_body(&self) -> EncodeResult<Vec<u8>>
    where
        Self: Sized;

    fn native_model_encode_downgrade_body(self, version: u32) -> Result<Vec<u8>>
    where
        Self: Sized;

    fn native_model_encode(&self) -> Result<Vec<u8>>
    where
        Self: Sized,
    {
        let mut data = self.native_model_encode_body()?;
        let data = crate::native_model_encode(
            &mut data,
            Self::native_model_id(),
            Self::native_model_version(),
        );
        Ok(data)
    }

    fn native_model_encode_downgrade(self, version: u32) -> Result<Vec<u8>>
    where
        Self: Sized,
    {
        let version = version.clone();
        let mut data = self.native_model_encode_downgrade_body(version)?;
        let data = crate::native_model_encode(&mut data, Self::native_model_id(), version);
        Ok(data)
    }
}
