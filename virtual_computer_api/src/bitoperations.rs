use bitvec::order::BitOrder;
use bitvec::view::BitView;
use bitvec::{slice::BitSlice,order::Lsb0};
use std::mem::size_of;
use std::ops::Range;
use std::sync::atomic::{AtomicU8, Ordering};
//Helper routines
fn get_bit_slice<T,O>(data: &[T])->&BitSlice<u8,O>
where O: BitOrder
{
    let ptr_slice: &[u8];
    unsafe {
        let raw_ptr_range: Range<*const T>=data.as_ptr_range();
        let start: *const u8=raw_ptr_range.start.cast();
        let real_end: *const u8=raw_ptr_range.end.sub(1).cast();
        let end: *const u8=real_end.add(1);
        ptr_slice=std::slice::from_ptr_range(start..end);
    }
    return ptr_slice.view_bits::<O>();
}
fn get_bit_slice_mut<T,O>(data: &mut [T])->&mut BitSlice<u8,O>
where O: BitOrder
{
    let ptr_slice: &mut [u8];
    unsafe {
        let raw_ptr_range: Range<*mut T>=data.as_mut_ptr_range();
        let start: *mut u8=raw_ptr_range.start.cast();
        let real_end: *mut u8=raw_ptr_range.end.sub(1).cast();
        let end: *mut u8=real_end.add(1);
        ptr_slice=std::slice::from_mut_ptr_range(start..end);
    }
    return ptr_slice.view_bits_mut::<O>();
}
fn get_atomic_bcav<'r,O>(bits:&mut BitSlice<u8,O>,nr: usize)->(&'r AtomicU8,u8)
where O: BitOrder
{
    let bit_ptr=unsafe{bits.as_mut_bitptr().add(nr)}.raw_parts();
    let address=bit_ptr.0.to_mut();
    let bitmask=!(bit_ptr.1.select::<O>().into_inner());
    let mut value=unsafe{AtomicU8::from_ptr(address)};
    return (value,bitmask)
}
//Structure holding these operations, bit operations inspired (stolen from) by qemu
pub struct BitOperations;
impl BitOperations {
    //set a bit in memory, in the provided bit order
    pub fn set_bit<T,O>(nr: usize,data:&mut [T],atomic:bool)
    where O: BitOrder
    {
        let data: &mut BitSlice<u8, O>=get_bit_slice_mut::<T,O>(data);
        assert!(data.len()-1>=nr);
        if atomic {
            let (value,bitmask)=get_atomic_bcav(data,nr);
            value.fetch_or(bitmask,Ordering::SeqCst);
        } else {
            data.set(nr, true);
        }
    }
    //set bits in raw bits slice
    pub fn set_bit_in_raw_bits<O>(nr: usize,data:&mut BitSlice<u8,O>,atomic:bool)
    where O: BitOrder
    {
        assert!(data.len()-1>=nr);
        if atomic {
            let (value,bitmask)=get_atomic_bcav(data,nr);
            value.fetch_or(bitmask,Ordering::SeqCst);
        } else {
            data.set(nr, true);
        }
    }
    //clear a bit in memory
    pub fn clear_bit<T,O>(nr: usize,data:&mut [T],atomic:bool)
    where O: BitOrder
    {
        let data: &mut BitSlice<u8, O>=get_bit_slice_mut::<T,O>(data);
        assert!(data.len()-1>=nr);
        if atomic {
            let (value,bitmask)=get_atomic_bcav(data,nr);
            value.fetch_and(bitmask,Ordering::SeqCst);
        } else {
            data.set(nr, false);
        }
    }
    //clear bits in raw bits slice
    pub fn clear_bit_in_raw_bits<O>(nr: usize,data:&mut BitSlice<u8,O>,atomic:bool)
    where O: BitOrder
    {
        assert!(data.len()-1>=nr);
        if atomic {
            let (value,bitmask)=get_atomic_bcav(data,nr);
            value.fetch_and(bitmask,Ordering::SeqCst);
        } else {
            data.set(nr, false);
        }
    }
    //flip a bit
    pub fn change_bit<T,O>(nr: usize,data:&mut [T],atomic:bool)
    where O: BitOrder
    {
        let data: &mut BitSlice<u8, O>=get_bit_slice_mut::<T,O>(data);
        assert!(data.len()-1>=nr);
        if atomic {
            let (value,bitmask)=get_atomic_bcav(data,nr);
            value.fetch_xor(bitmask,Ordering::SeqCst);
        } else {
            let org_value=*data.get(nr).unwrap();
            data.set(nr, !org_value);
        }
    }
    //flip a bit in bits slice
    pub fn change_bit_in_raw_bits<O>(nr: usize,data:&mut BitSlice<u8,O>,atomic:bool)
    where O: BitOrder
    {
        assert!(data.len()-1>=nr);
        if atomic {
            let (value,bitmask)=get_atomic_bcav(data,nr);
            value.fetch_xor(bitmask,Ordering::SeqCst);
        } else {
            let org_value=*data.get(nr).unwrap();
            data.set(nr, !org_value);
        }
    }
    //see if bit is set
    pub fn test_bit<T,O>(nr: usize,data:&[T])->bool 
    where O: BitOrder
    {
        let data: &BitSlice<u8, O>=get_bit_slice(&data);
        assert!(data.len()-1>=nr);
        return *data.get(nr).unwrap()
    }
    //see if bit is set in raw bits
    pub fn test_bit_in_raw_bits<O>(nr: usize,data:&BitSlice<u8,O>)->bool 
    where O: BitOrder
    {
        assert!(data.len()-1>=nr);
        return *data.get(nr).unwrap()
    }
    //see if bit is set and set bit
    pub fn test_and_set_bit<T,O>(nr: usize,data:&mut [T],atomic:bool)->bool 
    where O: BitOrder
    {
        let data: &mut BitSlice<u8, O>=get_bit_slice_mut(data);
        assert!(data.len()-1>=nr);
        let status=*data.get(nr).unwrap();
        Self::set_bit_in_raw_bits::<O>(nr, data, atomic);
        return status;
    }
    //see if bit is set and set bit in raw bits
    pub fn test_and_set_bit_in_raw_bits<O>(nr: usize,data:&mut BitSlice<u8,O>,atomic:bool)->bool 
    where O: BitOrder
    {
        assert!(data.len()-1>=nr);
        let status=*data.get(nr).unwrap();
        Self::set_bit_in_raw_bits::<O>(nr, data, atomic);
        return status;
    }
    //see if bit is set and clear bit
    pub fn test_and_clear_bit<T,O>(nr: usize,data:&mut [T],atomic:bool)->bool
    where O: BitOrder
    {
        let data: &mut BitSlice<u8, O>=get_bit_slice_mut(data);
        assert!(data.len()-1>=nr);
        let status=*data.get(nr).unwrap();
        Self::clear_bit_in_raw_bits(nr, data, atomic);
        return status;
    }
    //see if bit is set and clear bit in raw bitset
    pub fn test_and_clear_bit_in_raw_bits<O>(nr: usize,data:&mut BitSlice<u8,O>,atomic:bool)->bool
    where O: BitOrder
    {
        assert!(data.len()-1>=nr);
        let status=*data.get(nr).unwrap();
        Self::clear_bit_in_raw_bits::<O>(nr, data, atomic);
        return status;
    }
    //see if bit is set and change bit
    pub fn test_and_change_bit<T,O>(nr: usize,data:&mut [T],atomic:bool)->bool
    where O: BitOrder
    {
        let data: &mut BitSlice<u8, O>=get_bit_slice_mut(data);
        assert!(data.len()-1>=nr);
        let status=*data.get(nr).unwrap();
        Self::change_bit_in_raw_bits::<O>(nr, data, atomic);
        return status;
    }
    
    //see if bit is set and change bit in raw bitset
    pub fn test_and_change_bit_in_raw_bits<O>(nr: usize,data:&mut BitSlice<u8,O>,atomic:bool)->bool
    where O: BitOrder
    {
        assert!(data.len()-1>=nr);
        let status=*data.get(nr).unwrap();
        Self::change_bit_in_raw_bits::<O>(nr, data, atomic);
        return status;
    }
    //return last set bit in a memory range
    pub fn find_last_bit<T,O>(data:&[T])->usize
    where O: BitOrder
    {
        let bit_data:&BitSlice<u8,O>=get_bit_slice(data);
        for nr in (0..bit_data.len()).rev() {
            if *bit_data.get(nr).unwrap() {
                return nr;
            }
        }
        return data.len();
    }
    //return last cleared bit in a memory range
    pub fn find_last_zero_bit<T,O>(data: &[T])->usize
    where O: BitOrder
    {
        let bit_data:&BitSlice<u8,O>=get_bit_slice(data);
        for nr in (0..data.len()).rev() {
            if !(*bit_data.get(nr).unwrap()) {
                return nr;
            }
        }
        return data.len(); 
    }
    //find next set bit
    pub fn find_next_bit<T,O>(data: &[T],offset:usize)->usize
    where O: BitOrder
    {
        let bit_data:&BitSlice<u8,O>=get_bit_slice(data);
        if offset>=data.len() {
            return data.len();
        }
        for nr in offset..data.len() {
            if Self::test_bit::<T,O>(nr,data) {
                return nr;
            }
        }
        return data.len() 
    }
    //find next cleared bit
    pub fn find_next_zero_bit<T,O>(data: &[T],offset:usize)->usize
    where O: BitOrder
    {
        let bit_data:&BitSlice<u8,O>=get_bit_slice(data);
        if offset>=data.len() {
            return data.len();
        }
        for nr in offset..data.len() {
            if !Self::test_bit::<T,O>(nr,data) {
                return nr;
            }
        }
        return data.len() 
    }
    //find first set bit
    pub fn find_first_bit<T,O>(data:&[T])->usize
    where O: BitOrder
    {
        for nr in 0..data.len() {
            if Self::test_bit::<T,O>(nr,data) {
                return nr;
            }
        }
        return data.len() 
    }
    //find first cleared bit
    pub fn find_first_zero_bit<T,O>(data:&[T])->usize
    where O: BitOrder
    {
        for nr in 0..data.len() {
            if !Self::test_bit::<T,O>(nr,data) {
                return nr;
            }
        }
        return data.len();
    }
    //rotate 8 bit value left
    #[inline(always)]
    pub fn rol8(word:u8,n:u32)->u8 {
        word.rotate_left(n)
    }
    //rotate 8 bit value right
    #[inline(always)]
    pub fn ror8(word:u8,n:u32)->u8 {
        word.rotate_right(n)
    }
    //rotate 16 bit value left
    #[inline(always)]
    pub fn rol16(word:u16,n:u32)->u16 {
        word.rotate_left(n)
    }
    //rotate 16 bit value right
    #[inline(always)]
    pub fn ror16(word:u16,n:u32)->u16 {
        word.rotate_right(n)
    }
    //rotate 32 bit value left
    #[inline(always)]
    pub fn rol32(word:u32,n:u32)->u32 {
        word.rotate_left(n)
    }
    //rotate 32 bit value right
    #[inline(always)]
    pub fn ror32(word:u32,n:u32)->u32 {
        word.rotate_right(n)
    }
    //rotate 64 bit value left
    #[inline(always)]
    pub fn rol64(word:u64,n:u32)->u64 {
        word.rotate_left(n)
    }
    //rotate 64 bit value right
    #[inline(always)]
    pub fn ror64(word:u64,n:u32)->u64 {
        word.rotate_right(n)
    }
    //swap 16 bit halfwords in a 32 bit word
    #[inline(always)]
    pub fn hswap32(value:u32)->u32 {
        return value.rotate_left(16)
    }
    //swap 16 bit halfwords in a 64 bit word
    pub fn hswap64(mut value: u64)->u64 {
        let other_bitmask=0x0000ffff0000ffff;
        value=value.rotate_left(32);
        return ((value & other_bitmask) << 16) | ((value >> 16) & other_bitmask);
    }
    //swap 32 bit words in a 64 bit word
    #[inline(always)]
    pub fn wswap64(value: u64)->u64 {
        return value.rotate_left(32);
    }
    //extract a value from a 32 bit number
    pub fn extract32(value:u32,start:u32,length:u32)->u32 {
        assert!(length>0&&length<=32);
        return (value>>start)&(2_u32.pow(length-1)&(2_u32.pow(length-1)-1))
    }
    //extract a value from a 8 bit number
    pub fn extract8(value:u8,start:u32,length:u32)->u8 {
        assert!(length>0&&length<=8);
        return (value>>start)&(2_u8.pow(length-1)&(2_u8.pow(length-1)-1))
    }
    //extract a value from a 16 bit number
    pub fn extract16(value:u16,start:u32,length:u32)->u16 {
        assert!(length>0&&length<=16);
        return (value>>start)&(2_u16.pow(length-1)&(2_u16.pow(length-1)-1))
    }
    //extract a value from a 64 bit number
    pub fn extract64(value:u64,start:u32,length:u32)->u64 {
        assert!(length>0&&length<=64);
        return (value>>start)&(2_u64.pow(length-1)&(2_u64.pow(length-1)-1));
    }
    //extract a signed extended value from a 32 bit number
    pub fn sextract32(value:u32,start:u32,length:u32)->i32 {
        assert!(length>0&&length<=32);
        return ((value<<(32-length-start))>>(32-length)).try_into().unwrap()
    }
    //extract a signed extended value from a 8 bit number
    pub fn sextract8(value:u8,start:u32,length:u32)->i8 {
        assert!(length>0&&length<=8);
        return ((value<<(8-length-start))>>(8-length)).try_into().unwrap()
    }
    //extract a signed extended value from a 16 bit number
    pub fn sextract16(value:u16,start:u32,length:u32)->i16 {
        assert!(length>0&&length<=16);
        return ((value<<(16-length-start))>>(16-length)).try_into().unwrap()
    }
    //extract a signed extended value from a 64 bit number
    pub fn sextract64(value:u64,start:u32,length:u32)->i64 {
        assert!(length>0&&length<=64);
        return ((value<<(64-length-start))>>(64-length)).try_into().unwrap()
    }
    //deposit bits of a 32 bit value into another.
    pub fn deposit32(mut value:u32,start:u32,length:u32,field_value:u32)->u32 {
        assert!(length>0&&length<=32);
        {
            let other_bitmask=(u32::MAX>>(32-length))<<start;
            value&=!other_bitmask;
            value|=field_value&other_bitmask;
        }
        return value;
    }
    //deposit bits of a 64 bit value into another.
    pub fn deposit64(mut value:u64,start:u32,length:u32,field_value:u64)->u64 {
        assert!(length>0&&length<=64);
        {
            let other_bitmask=(u64::MAX>>(64-length))<<start;
            value&=!other_bitmask;
            value|=field_value&other_bitmask;
        }
        return value;
    }
    
    //deposit bits of a 16 bit value into another.
    pub fn deposit16(mut value:u16,start:u32,length:u32,field_value:u16)->u16 {
        assert!(length>0&&length<=16);
        {
            let other_bitmask=(u16::MAX>>(16-length))<<start;
            value&=!other_bitmask;
            value|=field_value&other_bitmask;
        }
        return value;
    }
    //deposit bits of a 8 bit value into another.
    pub fn deposit8(mut value:u8,start:u32,length:u32,field_value:u8)->u8 {
        assert!(length>0&&length<=8);
        {
            let other_bitmask=(u8::MAX>>(8-length))<<start;
            value&=!other_bitmask;
            value|=field_value&other_bitmask;
        }
        return value;
    }
    //return the value where the lower half is spread out into the odd bits in the word, and the even bits are zeroed (not by 0 based index)
    pub fn half_shuffle32(mut value:u32)->u32 {
        value = ((value & 0xFF00) << 8) | (value & 0x00FF);
        value = ((value << 4) | value) & 0x0F0F0F0F;
        value = ((value << 2) | value) & 0x33333333;
        value = ((value << 1) | value) & 0x55555555;
        return value;
    }
    //64 bit variant
    pub fn half_shuffle64(mut value:u64)->u64 {
        value = ((value & 0xFFFF0000) << 16) | (value & 0xFFFF);
        value = ((value << 8) | value) & 0x00FF00FF00FF00FF;
        value = ((value << 4) | value) & 0x0F0F0F0F0F0F0F0F;
        value= ((value << 2) | value) & 0x3333333333333333;
        value = ((value << 1) | value) & 0x5555555555555555;
        return value;
    }
    //return the value where all the odd bits are compressed down into the low half of the word, and the high half is zeroed
    pub fn half_unshuffle32(mut value: u32)->u32 {
        value &= 0x55555555;
        value = ((value >> 1) | value) & 0x33333333;
        value = ((value >> 2) | value) & 0x0F0F0F0F;
        value = ((value >> 4) | value) & 0x00FF00FF;
        value = ((value >> 8) | value) & 0x0000FFFF;
        return value
    }
    //64 bit variant
    pub fn half_unshuffle64(mut value: u64)->u64 {
        value &= 0x5555555555555555;
        value = ((value >> 1) | value) & 0x3333333333333333;
        value = ((value >> 2) | value) & 0x0F0F0F0F0F0F0F0F;
        value = ((value >> 4) | value) & 0x00FF00FF00FF00FF;
        value = ((value >> 8) | value) & 0x0000FFFF0000FFFF;
        value = ((value >> 16) | value) & 0x00000000FFFFFFFF;
        return value
    }
}
