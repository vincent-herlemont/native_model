use zerocopy::little_endian::U32;
use zerocopy::{AsBytes, FromBytes, FromZeroes};

#[derive(FromZeroes, FromBytes, AsBytes, Debug)]
#[repr(C)]
pub struct Header {
    pub(crate) id: U32,
    pub(crate) version: U32,
}
