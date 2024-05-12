use crate::header::Header;
use zerocopy::little_endian::U32;
use zerocopy::{AsBytes, ByteSlice, ByteSliceMut, Ref};

#[derive(Debug)]
pub struct Wrapper<T: ByteSlice> {
    header: Ref<T, Header>, // Deprecated: Rename LayoutVerified to Ref #203
    value: T,
}

impl<T: ByteSlice> Wrapper<T> {
    pub fn deserialize(packed: T) -> Option<Self> {
        let (header_lv, rest) = Ref::<_, Header>::new_from_prefix(packed)?;
        let native_model = Self {
            header: header_lv,
            value: rest,
        };
        Some(native_model)
    }

    pub const fn value(&self) -> &T {
        &self.value
    }

    pub fn get_type_id(&self) -> u32 {
        self.header.id.get()
    }

    pub fn get_id(&self) -> u32 {
        self.header.id.get()
    }

    pub fn get_version(&self) -> u32 {
        self.header.version.get()
    }
}

impl<T: ByteSliceMut> Wrapper<T> {
    pub fn set_type_id(&mut self, type_id: u32) {
        self.header.id = U32::new(type_id);
    }

    pub fn set_version(&mut self, version: u32) {
        self.header.version = U32::new(version);
    }
}

pub fn native_model_encode(data: &mut Vec<u8>, type_id: u32, version: u32) -> Vec<u8> {
    let header = Header {
        id: U32::new(type_id),
        version: U32::new(version),
    };
    let mut header = header.as_bytes().to_vec();
    header.append(data);
    header
}

#[cfg(test)]
mod tests {
    use crate::{native_model_encode, Wrapper};

    #[test]
    fn native_model_deserialize_with_body() {
        let mut data = vec![0u8; 8];
        let data = native_model_encode(&mut data, 200000, 100000);
        assert_eq!(data.len(), 16);
        let model = Wrapper::deserialize(&data[..]).unwrap();
        assert_eq!(model.get_type_id(), 200000);
        assert_eq!(model.get_version(), 100000);
        assert_eq!(model.value().len(), 8);
    }
}
