#[allow(non_camel_case_types, dead_code)]
pub mod be_types {
    pub type i16_be = zerocopy::byteorder::big_endian::I16;
    pub type i32_be = zerocopy::byteorder::big_endian::I32;
    pub type i64_be = zerocopy::byteorder::big_endian::I64;
    pub type u16_be = zerocopy::byteorder::big_endian::U16;
    pub type u32_be = zerocopy::byteorder::big_endian::U32;
    pub type u64_be = zerocopy::byteorder::big_endian::U64;
}
