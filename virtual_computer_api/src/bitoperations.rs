use bitvec::order::BitOrder;
use bitvec::store::BitStore;
use bitvec::view::BitView;
use bitvec::{slice::BitSlice,boxed::BitBox};
use std::ops::Range;
use std::mem::size_of;
use std::sync::atomic::{AtomicU8,AtomicU16, AtomicU32, AtomicU64, Ordering};
//Helper routines
//Get the bit slice from the a slice of a certain type in a certain order
fn get_bit_slice<T,BT,O>(data: &[T])->&BitSlice<BT,O>
where
    BT: BitStore,
    O: BitOrder
{
    let ptr_slice: &[u8];
    unsafe {
        let raw_ptr_range: Range<*const T>=data.as_ptr_range();
        let start: *const BT=raw_ptr_range.start.cast();
        let real_end: *const BT=raw_ptr_range.end.sub(1).cast();
        let end: *const BT=real_end.add(1);
        ptr_slice=std::slice::from_ptr_range(start..end);
    }
    return ptr_slice.view_bits::<O>();
}
//Get the mutable bit slice from the a slice of a certain type in a certain order
fn get_bit_slice_mut<T,BT,O>(data: &mut [T])->&mut BitSlice<BT,O>
where
    BT: BitStore,
    O: BitOrder
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
//Get the values that help control the bits in an atomic operation
fn get_atomic_bit_control_values<'r,BT,O>(bits:&mut BitSlice<BT,O>,nr: usize)->(&'r AtomicU8,u8)
where
    BT: BitStore,
    O: BitOrder
{
    let bit_ptr=unsafe{bits.as_mut_bitptr().add(nr)}.raw_parts();
    let address=bit_ptr.0.to_mut();
    let bitmask=!(bit_ptr.1.select::<O>().into_inner());
    let value=unsafe{AtomicU8::from_ptr(address)};
    return (value,bitmask)
}
//Structure holding these operations, bit operations inspired (stolen from) by qemu
pub struct BitOperations;
impl BitOperations {
    //set a bit in memory, in the provided bit order
    pub fn set_bit<T,O>(nr: usize,data:&mut [T],atomic:bool)
    where O: BitOrder
    {
        let data: &mut BitSlice<u32, O>=get_bit_slice_mut::<T,u32,O>(data);
        Self::set_bit_in_raw_bits<u32,O>(nr,data,atomic);
    }
    //set bits in raw bits slice
    pub fn set_bit_in_raw_bits<BT,O>(nr: usize,data:&mut BitSlice<BT,O>,atomic:bool)
    where
        BT: BitStore,
        O: BitOrder
    {
        assert!(data.len()-1>=nr);
        if atomic {
            let (value,bitmask)=get_atomic_bit_control_values(data,nr);
            value.fetch_or(bitmask,Ordering::SeqCst);
        } else {
            data.set(nr, true);
        }
    }
    //clear a bit in memory
    pub fn clear_bit<T,O>(nr: usize,data:&mut [T],atomic:bool)
    where O: BitOrder
    {
        let data: &mut BitSlice<u32, O>=get_bit_slice_mut::<T,u32,O>(data);
        Self::clear_bit_in_raw_bits<u32,O>(nr, data, atomic);
    }
    //clear bits in raw bits slice
    pub fn clear_bit_in_raw_bits<BT,O>(nr: usize,data:&mut BitSlice<BT,O>,atomic:bool)
    where 
        BT: BitStore,
        O: BitOrder
    {
        assert!(data.len()-1>=nr);
        if atomic {
            let (value,bitmask)=get_atomic_bit_control_values(data,nr);
            value.fetch_and(bitmask,Ordering::SeqCst);
        } else {
            data.set(nr, false);
        }
    }
    //flip a bit
    pub fn change_bit<T,O>(nr: usize,data:&mut [T],atomic:bool)
    where O: BitOrder
    {
        let data: &mut BitSlice<u32, O>=get_bit_slice_mut::<T,u32,O>(data);
        Self::change_bit_in_raw_bits(nr, data, atomic);
    }
    //flip a bit in bits slice
    pub fn change_bit_in_raw_bits<BT,O>(nr: usize,data:&mut BitSlice<BT,O>,atomic:bool)
    where
        BT: BitStore,
        O: BitOrder
    {
        assert!(data.len()-1>=nr);
        if atomic {
            let (value,bitmask)=get_atomic_bit_control_values(data,nr);
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
        Self::test_bit_in_raw_bits(nr, data)
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
        Self::test_and_set_bit_in_raw_bits(nr,data,atomic)
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
        Self::test_and_clear_bit_in_raw_bits(nr, data, atomic)
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
        Self::test_and_change_bit_in_raw_bits(nr, data, atomic)
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
    pub fn find_last_set_bit<T,O>(data:&[T])->usize
    where O: BitOrder
    {
        let bit_data:&BitSlice<u8,O>=get_bit_slice(data);
        Self::find_last_set_bit_in_raw_bits(bit_data)
    }
    //return last set bit index in a memory range in raw bitset
    pub fn find_last_set_bit_in_raw_bits<O>(bit_data:&BitSlice<u8,O>)->usize
    where O: BitOrder
    {
        for nr in (0..bit_data.len()).rev() {
            if *bit_data.get(nr).unwrap() {
                return nr;
            }
        }
        return bit_data.len();
    }
    //return last cleared bit in a memory range
    pub fn find_last_zero_bit<T,O>(data: &[T])->usize
    where O: BitOrder
    {
        let bit_data:&BitSlice<u8,O>=get_bit_slice(data);
        Self::find_last_zero_bit_in_raw_bits(bit_data)
    }
    //return last cleared bit in a memory range in raw bitset
    pub fn find_last_zero_bit_in_raw_bits<O>(bit_data:&BitSlice<u8,O>)->usize
    where O: BitOrder
    {
        for nr in (0..bit_data.len()).rev() {
            if !(*bit_data.get(nr).unwrap()) {
                return nr;
            }
        }
        return bit_data.len();
    }
    //find next set bit
    pub fn find_next_set_bit<T,O>(data: &[T],offset:usize)->usize
    where O: BitOrder
    {
        let bit_data:&BitSlice<u8,O>=get_bit_slice(data);
        Self::find_next_set_bit_in_raw_bits(bit_data,offset)
    }
    //find next set bit in raw bitset
    pub fn find_next_set_bit_in_raw_bits<O>(bit_data: &BitSlice<u8,O>,offset:usize)->usize
    where O: BitOrder
    {
        if offset>=bit_data.len() {
            return bit_data.len();
        }
        for nr in offset..bit_data.len() {
            if *bit_data.get(nr).unwrap() {
                return nr;
            }
        }
        return bit_data.len() 
    }
    //find next cleared bit
    pub fn find_next_zero_bit<T,O>(data: &[T],offset:usize)->usize
    where O: BitOrder
    {
        let bit_data:&BitSlice<u8,O>=get_bit_slice(data);
        Self::find_next_zero_bit_in_raw_bits(bit_data,offset)
    }
    //find next cleared bit in 
    pub fn find_next_zero_bit_in_raw_bits<O>(bit_data: &BitSlice<u8,O>,offset:usize)->usize
    where O: BitOrder
    {
        if offset>=bit_data.len() {
            return bit_data.len();
        }
        for nr in offset..bit_data.len() {
            if !(*bit_data.get(nr).unwrap()) {
                return nr;
            }
        }
        return bit_data.len() 
    }
    //find first set bit
    pub fn find_first_set_bit<T,O>(data:&[T])->usize
    where O: BitOrder
    {
        let bit_data: &BitSlice<u8,O>=get_bit_slice(data);
        Self::find_first_set_bit_in_raw_bits::<T,O>(bit_data)
    }
    //find first set bit in raw bitset
    pub fn find_first_set_bit_in_raw_bits<T,O>(bit_data:&BitSlice<u8,O>)->usize
    where O: BitOrder
    {
        for nr in 0..bit_data.len() {
            if *bit_data.get(nr).unwrap() {
                return nr;
            }
        }
        return bit_data.len() 
    }
    //find first cleared bit
    pub fn find_first_zero_bit<T,O>(data:&[T])->usize
    where O: BitOrder
    {
        let bit_data: &BitSlice<u8,O>=get_bit_slice(data);
        Self::find_first_zero_bit_in_raw_bits::<T,O>(bit_data)
    }
    //find first cleared bit in raw bitset
    pub fn find_first_zero_bit_in_raw_bits<T,O>(bit_data:&BitSlice<u8,O>)->usize
    where O: BitOrder
    {
        for nr in 0..bit_data.len() {
            if !(*bit_data.get(nr).unwrap()) {
                return nr;
            }
        }
        return bit_data.len();
    }
    //rotate 8 bit value left, assumes leftmost bit is highest value bit
    #[inline(always)]
    pub fn rotate_left_u8(word:u8,n:u32)->u8 {
        word.rotate_left(n)
    }
    //rotate referenced 8 bit value left, assumes leftmost bit is highest value bit
    pub fn rotate_left_u8_direct(word:&mut u8,n:u32,atomic:bool) {
        if atomic {
            let value=word.clone();
            let atomic_value=AtomicU8::from_mut(word);
            atomic_value.swap(value.rotate_left(n),Ordering::SeqCst);
        } else {
            *word=word.rotate_left(n);
        }
    }
    //rotate 8 bit value right, assumes rightmost bit is lowest value bit
    #[inline(always)]
    pub fn rotate_right_u8(word:u8,n:u32)->u8 {
        word.rotate_right(n)
    }
    //rotate referenced 8 bit value right, assumes rightmost bit is lowest value bit
    pub fn rotate_right_u8_direct(word:&mut u8,n:u32,atomic:bool) {
        if atomic {
            let value=word.clone();
            let atomic_value=AtomicU8::from_mut(word);
            atomic_value.swap(value.rotate_right(n),Ordering::SeqCst);
        } else {
            *word=word.rotate_right(n);
        }
    }
    //rotate 16 bit value left, assumes leftmost bit is highest value bit
    #[inline(always)]
    pub fn rotate_left_u16(word:u16,n:u32)->u16 {
        word.rotate_left(n)
    }
    //rotate referenced 16 bit value left, assumes leftmost bit is highest value bit
    pub fn rotate_left_u16_direct(word:&mut u16,n:u32,atomic:bool) {
        if atomic {
            let value=word.clone();
            let atomic_value=AtomicU16::from_mut(word);
            atomic_value.swap(value.rotate_left(n),Ordering::SeqCst);
        } else {
            *word=word.rotate_left(n);
        }
    }
    //rotate 16 bit value right, assumes rightmost bit is lowest value bit
    #[inline(always)]
    pub fn rotate_right_u16(word:u16,n:u32)->u16 {
        word.rotate_right(n)
    }
    //rotate referenced 16 bit value right, assumes rightmost bit is lowest value bit
    pub fn rotate_right_u16_direct(word:&mut u16,n:u32,atomic:bool) {
        if atomic {
            let value=word.clone();
            let atomic_value=AtomicU16::from_mut(word);
            atomic_value.swap(value.rotate_right(n),Ordering::SeqCst);
        } else {
            *word=word.rotate_right(n);
        }
    }
    //rotate 32 bit value left, assumes leftmost bit is highest value bit
    #[inline(always)]
    pub fn rotate_left_u32(word:u32,n:u32)->u32 {
        word.rotate_left(n)
    }
    //rotate referenced 32 bit value left, assumes leftmost bit is highest value bit
    pub fn rotate_left_u32_direct(word:&mut u32,n:u32,atomic:bool) {
        if atomic {
            let value=word.clone();
            let atomic_value=AtomicU32::from_mut(word);
            atomic_value.swap(value.rotate_left(n),Ordering::SeqCst);
        } else {
            *word=word.rotate_left(n);
        }
    }
    //rotate 32 bit value right
    #[inline(always)]
    pub fn rotate_right_u32(word:u32,n:u32)->u32 {
        word.rotate_right(n)
    }
    //rotate referenced 32 bit value right, assumes rightmost bit is lowest value bit
    pub fn rotate_right_u32_direct(word:&mut u32,n:u32,atomic:bool) {
        if atomic {
            let value=word.clone();
            let atomic_value=AtomicU32::from_mut(word);
            atomic_value.swap(value.rotate_right(n),Ordering::SeqCst);
        } else {
            *word=word.rotate_right(n);
        }
    }
    //rotate 64 bit value left
    #[inline(always)]
    pub fn rotate_left_u64(word:u64,n:u32)->u64 {
        word.rotate_left(n)
    }
    //rotate referenced 64 bit value left, assumes leftmost bit is highest value bit
    pub fn rotate_left_u64_direct(word:&mut u64,n:u32,atomic:bool) {
        if atomic {
            let value=word.clone();
            let atomic_value=AtomicU64::from_mut(word);
            atomic_value.swap(value.rotate_left(n),Ordering::SeqCst);
        } else {
            *word=word.rotate_left(n);
        }
    }
    //rotate 64 bit value right
    #[inline(always)]
    pub fn rotate_right_u64(word:u64,n:u32)->u64 {
        word.rotate_right(n)
    }
    //rotate referenced 64 bit value right, assumes rightmost bit is lowest value bit
    pub fn rotate_right_u64_direct(word:&mut u64,n:u32,atomic:bool) {
        if atomic {
            let value=word.clone();
            let atomic_value=AtomicU64::from_mut(word);
            atomic_value.swap(value.rotate_right(n),Ordering::SeqCst);
        } else {
            *word=word.rotate_left(n);
        }
    }
    //rotate a bitset left
    pub fn rotate_left_bitset<T,O>(bits:&BitSlice<T,O>,n:usize)->BitBox<T,O>
    where
    T: BitStore,
    O: BitOrder
    {
        let mut copied_bitslice=BitBox::from_bitslice(bits);
        copied_bitslice.rotate_left(n);
        return copied_bitslice;
    }
    //rotate a referenced bitset left directly
    pub fn rotate_left_bitset_direct<T,O>(bits:&mut BitSlice<T,O>,n:usize)
    where
    T: BitStore,
    O: BitOrder
    {
        bits.rotate_left(n);
    }
    //rotate a bitset right
    pub fn rotate_right_bitset<T,O>(bits:&BitSlice<T,O>,n:usize)->BitBox<T,O>
    where
    T: BitStore,
    O: BitOrder
    {
        let mut copied_bitslice=BitBox::from_bitslice(bits);
        copied_bitslice.rotate_right(n);
        return copied_bitslice;
    }
    //rotate a referenced bitset right directly
    pub fn rotate_right_bitset_direct<T,O>(bits:&mut BitSlice<T,O>,n:usize)
    where
    T: BitStore,
    O: BitOrder
    {
        bits.rotate_right(n);
    }
    //swap 16 bit halfwords in a 32 bit word
    #[inline(always)]
    pub fn halfword_swap_u32(value:u32)->u32 {
        return value.rotate_left(16)
    }
    //swap 16 bit halfwords in a 64 bit word
    pub fn halfword_swap_u64(mut value: u64)->u64 {
        const OTHER_BITMASK:u64=0x0000ffff0000ffff;
        value=value.rotate_left(32);
        return ((value & OTHER_BITMASK) << 16) | ((value >> 16) & OTHER_BITMASK);
    }
    //swap 32 bit words in a 64 bit word
    #[inline(always)]
    pub fn word_swap_u64(value: u64)->u64 {
        return value.rotate_left(32);
    }
    //Extract bits from a single value
    pub fn extract_bits<T,O>(value: &T,start:usize,length:usize)->BitBox<u8,O>
    where O: BitOrder
    {
        let type_size=size_of::<T>()<<3;
        assert!(type_size>start&&length<=type_size-start);
        Self::extract_bits_of_bitset_unchecked::<T,O>(get_bit_slice::<T,O>(std::slice::from_ref(value)),start,length)
    }
    //Extract bits from a single value
    pub fn extract_bits_from_slice<T,O>(value: &[T],start:usize,length:usize)->BitBox<u8,O>
    where O: BitOrder
    {
        let array_size=(size_of::<T>()<<3)*value.len();
        assert!(array_size>start&&length<=array_size-start);
        let bit_slice=get_bit_slice::<T,O>(value);
        Self::extract_bits_of_bitset_unchecked::<T,O>(bit_slice,start,length)
    }
    //Extract bits from a bitset.
    pub fn extract_bits_of_bitset<O>(bits:&BitSlice<u8,O>,start:usize,length:usize)->BitBox<u8,O>
    where O: BitOrder
    {
        assert!(bits.len()>start&&length<=bits.len()-start);
        Self::extract_bits_of_bitset_unchecked::<T,O>(bits,start,length)
    }
    //Unchecked version of bit extraction
    fn extract_bits_of_bitset_unchecked<O>(bits:&BitSlice<u8,O>,start:usize,length:usize)->BitBox<u8,O>
    where O: BitOrder
    {
        let specific_bit_slice=&bits[start..length];
        BitBox::from_bitslice(specific_bit_slice)
    }
    /*pub fn sextract32(value:u32,start:u32,length:u32)->i32 {
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
    }*/
    //deposit bits of one owned bitset into another
    fn deposit_bits_of_bitset_unchecked<O>(value:&mut BitSlice<u8,O>,start:usize,bits:BitBox<u8,O>)
    where O: BitOrder
    {
        //
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
