use ux::{u24,i24};
use byteorder::{LittleEndian, BigEndian,ByteOrder};
#[inline]
pub fn load_unsigned_byte_with_host_pointer(ptr:&u8)->u8 {
    return *ptr;
}
#[inline]
pub fn load_signed_byte_with_host_pointer(ptr:&i8)->i8 {
    return *ptr;
}
#[inline]
pub fn load_unsigned_word_with_host_pointer_in_little_endian(ptr:&[u8])->u16 {
    return LittleEndian::read_u16(ptr);
}
#[inline]
pub fn load_signed_word_with_host_pointer_in_little_endian(ptr:&[u8])->i16 {
    return LittleEndian::read_i16(ptr);
}
#[inline]
pub fn load_unsigned_word_with_host_pointer_in_big_endian(ptr:&[u8])->u16 {
    return BigEndian::read_u16(ptr);
}
#[inline]
pub fn load_signed_word_with_host_pointer_in_big_endian(ptr:&[u8])->i16 {
    return BigEndian::read_i16(ptr);
}