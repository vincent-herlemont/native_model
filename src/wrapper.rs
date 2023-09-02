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

    pub fn value(&self) -> &T {
        &self.value
    }

    pub fn get_type_id(&self) -> u32 {
        self.header.type_id.get()
    }

    pub fn get_version(&self) -> u32 {
        self.header.version.get()
    }
}

impl<T: ByteSliceMut> Wrapper<T> {
    pub fn set_type_id(&mut self, type_id: u32) {
        self.header.type_id = U32::new(type_id);
    }

    pub fn set_version(&mut self, version: u32) {
        self.header.version = U32::new(version);
    }
}

pub fn native_model_encode(value: &mut Vec<u8>, type_id: u32, version: u32) {
    let header = Header {
        type_id: U32::new(type_id),
        version: U32::new(version),
    };
    let header = header.as_bytes();
    value.reserve(header.len());
    value.splice(..0, header.iter().cloned());

    // Try to do with unsafe code to improve performance but benchmark shows that it's the same
    //
    // // Add header to the beginning of the vector
    // unsafe {
    //     // get the raw pointer to the vector's buffer
    //     let ptr = value.as_mut_ptr();
    //
    //     // move the existing elements to the right
    //     ptr.offset(header.len() as isize)
    //         .copy_from_nonoverlapping(ptr, value.len());
    //
    //     // copy the elements from the header to the beginning of the vector
    //     ptr.copy_from_nonoverlapping(header.as_ptr(), header.len());
    //
    //     // update the length of the vector
    //     value.set_len(value.len() + header.len());
    // }
}

#[cfg(test)]
mod tests {
    use crate::{native_model_encode, Wrapper};

    #[test]
    fn native_model_deserialize_with_body() {
        let mut data = vec![0u8; 8];
        native_model_encode(&mut data, 200000, 100000);
        assert_eq!(data.len(), 16);
        let model = Wrapper::deserialize(&data[..]).unwrap();
        assert_eq!(model.get_type_id(), 200000);
        assert_eq!(model.get_version(), 100000);
        assert_eq!(model.value().len(), 8);
    }
}
