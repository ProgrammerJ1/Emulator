use ux::{u24,i24};
use crate::host_context::{HostContext,Endianness};
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
#[inline]
pub fn load_unsigned_word_with_host_pointer_in_host_endian(system_context:&HostContext,ptr:&[u8])->u16 {
    match system_context.endianess {
        Endianness::Little=>return load_unsigned_word_with_host_pointer_in_little_endian(ptr),
        Endianness::Big=>return load_unsigned_word_with_host_pointer_in_big_endian(ptr)
    }
}
#[inline]
pub fn load_signed_word_with_host_pointer_in_host_endian(system_context:&HostContext,ptr:&[u8])->i16 {
    match system_context.endianess {
        Endianness::Little=>return load_signed_word_with_host_pointer_in_little_endian(ptr),
        Endianness::Big=>return load_signed_word_with_host_pointer_in_big_endian(ptr)
    }
}
#[inline]
pub fn load_unsigned_24bit_with_host_pointer_in_little_endian(ptr:&[u8])->u24 {
    return u24::new(LittleEndian::read_u24(ptr));;
}
#[inline]
pub fn load_signed_24bit_with_host_pointer_in_little_endian(ptr:&[u8])->i24 {
    return i24::new(LittleEndian::read_i24(ptr));
}
#[inline]
pub fn load_unsigned_24bit_with_host_pointer_in_big_endian(ptr:&[u8])->u24 {
    return u24::new(BigEndian::read_u24(ptr));
}
#[inline]
pub fn load_signed_24bit_with_host_pointer_in_big_endian(ptr:&[u8])->i24 {
    return i24::new(BigEndian::read_i24(ptr));
}
#[inline]
pub fn load_unsigned_24bit_with_host_pointer_in_host_endian(system_context:&HostContext,ptr:&[u8])->u24 {
    match system_context.endianess {
        Endianness::Little=>load_unsigned_24bit_with_host_pointer_in_little_endian(ptr),
        Endianness::Big=>load_unsigned_24bit_with_host_pointer_in_big_endian(ptr)
    }
}
#[inline]
pub fn load_signed_24bit_with_host_pointer_in_host_endian(system_context:&HostContext,ptr:&[u8])->i24 {
    match system_context.endianess {
        Endianness::Little=>load_signed_24bit_with_host_pointer_in_little_endian(ptr),
        Endianness::Big=>load_signed_24bit_with_host_pointer_in_big_endian(ptr)
    }
}
