use std::mem::size_of;
use std::slice::SliceIndex;
use std::sync::atomic::AtomicPtr;
//Structure holding these operations
pub struct BitOperations;
impl BitOperations {
    //set a bit in memory
    pub fn set_bit(nr: u64,address:&mut u64) {
        let p=unsafe{std::ptr::from_mut(address).offset((nr/((size_of::<u64>()*8) as u64)) as isize).as_mut()}.unwrap();
        *p|=1<<(nr%((size_of::<u64>()*8) as u64));
    }
    //set a bit in memory atomically
    pub fn set_bit_atomically(nr: u64,address:AtomicPtr<u64>) {
        Self::set_bit(nr, unsafe{address.into_inner().as_mut()}.unwrap());
    }
    //clear a bit in memory
    pub fn clear_bit(nr: u64,address:&mut u64) {
        let p=unsafe{std::ptr::from_mut(address).offset((nr/((size_of::<u64>()*8) as u64)) as isize).as_mut()}.unwrap();
        *p|=!(1<<(nr%((size_of::<u64>()*8) as u64)));
    }
    //clear a bit in memory atomically
    pub fn clear_bit_atomically(nr: u64,address:AtomicPtr<u64>) {
        Self::clear_bit(nr, unsafe{address.into_inner().as_mut()}.unwrap());
    }
    //flip a bit
    pub fn change_bit(nr: u64,address:&mut u64) {
        let p=unsafe{std::ptr::from_mut(address).offset((nr/((size_of::<u64>()*8) as u64)) as isize).as_mut()}.unwrap();
        *p^=1<<(nr%((size_of::<u64>()*8) as u64));
    }
    //see test if bit is set and set bit
    pub fn test_and_set_bit(nr: u64,address:&mut u64)->bool {
        let p=unsafe{std::ptr::from_mut(address).offset((nr/((size_of::<u64>()*8) as u64)) as isize).as_mut()}.unwrap();
        let mut test=p.clone();
        test&=1<<(nr%((size_of::<u64>()*8) as u64));
        Self::set_bit(nr%((size_of::<u64>()*8) as u64), p);
        return test>0
    }
    //see test if bit is set and clear bit
    pub fn test_and_clear_bit(nr: u64,address:&mut u64)->bool {
        let p=unsafe{std::ptr::from_mut(address).offset((nr/((size_of::<u64>()*8) as u64)) as isize).as_mut()}.unwrap();
        let mut test=p.clone();
        test&=1<<(nr%((size_of::<u64>()*8) as u64));
        Self::clear_bit(nr%((size_of::<u64>()*8) as u64), p);
        return test>0
    }
    //see test if bit is set and change bit
    pub fn test_and_change_bit(nr: u64,address:&mut u64)->bool {
        let p=unsafe{std::ptr::from_mut(address).offset((nr/((size_of::<u64>()*8) as u64)) as isize).as_mut()}.unwrap();
        let mut test=p.clone();
        test&=1<<(nr%((size_of::<u64>()*8) as u64));
        Self::change_bit(nr%((size_of::<u64>()*8) as u64), p);
        return test>0
    }
    //see if bit is set
    pub fn test_bit(nr: u64,address:&u64)->bool {
        let p=unsafe{std::ptr::from_ref(address).offset((nr/((size_of::<u64>()*8) as u64)) as isize).as_ref()}.unwrap();
        let mut test=p.clone();
        test&=1<<(nr%((size_of::<u64>()*8) as u64));
        return test>0
    }
    //return last set bit in a memory range
    pub fn find_last_bit(address:&u64,size:u64)->u64 {
        for nr in (0..size).rev() {
            if Self::test_bit(nr,address) {
                return nr;
            }
        }
        return size 
    }
    //return last cleared bit in a memory range
    pub fn find_last_zero_bit(address:&u64,size:u64)->u64 {
        for nr in (0..size).rev() {
            if !Self::test_bit(nr,address) {
                return nr;
            }
        }
        return size 
    }
    //find next set bit
    pub fn find_next_bit(address:&u64,offset:u64,size:u64)->u64 {
        if (offset>=size) {
            return size;
        }
        for nr in (offset..size) {
            if Self::test_bit(nr,address) {
                return nr;
            }
        }
        return size 
    }
    //find next cleared bit
    pub fn find_next_zero_bit(address:&u64,offset:u64,size:u64)->u64 {
        if (offset>=size) {
            return size;
        }
        for nr in (offset..size) {
            if !Self::test_bit(nr,address) {
                return nr;
            }
        }
        return size 
    }
    //find first set bit
    pub fn find_first_zero_bit(address:&u64,size:u64)->u64 {
        if (offset>=size) {
            return size;
        }
        for nr in (0..size) {
            if Self::test_bit(nr,address) {
                return nr;
            }
        }
        return size 
    }
    //find first cleared bit
    pub fn find_first_zero_bit(address:&u64,size:u64)->u64 {
        if (offset>=size) {
            return size;
        }
        for nr in (0..size) {
            if !Self::test_bit(nr,address) {
                return nr;
            }
        }
        return size;
    }
    //rotate 8 bit value left
    pub fn rol8(word:u8,n:u32)->u8 {
        word.rotate_left(rotate)
    }
    //rotate 8 bit value right
    pub fn ror8(word:u8,n:u32)->u8 {
        word.rotate_right(rotate)
    }
    //rotate 16 bit value left
    pub fn rol16(word:u16,n:u32)->u16 {
        word.rotate_left(rotate)
    }
    //rotate 16 bit value right
    pub fn ror16(word:u16,n:u32)->u16 {
        word.rotate_right(rotate)
    }
    //rotate 32 bit value left
    pub fn rol32(word:u32,n:u32)->u32 {
        word.rotate_left(rotate)
    }
    //rotate 32 bit value right
    pub fn ror32(word:u32,n:u32)->u32 {
        word.rotate_right(rotate)
    }
    //rotate 64 bit value left
    pub fn rol64(word:u64,n:u32)->u64 {
        word.rotate_left(rotate)
    }
    //rotate 64 bit value right
    pub fn ror64(word:u64,n:u32)->u64 {
        word.rotate_right(rotate)
    }
    pub fn hswap32(value:u32)->u32 {
        return value.rotate_left(16)
    }
    pub fn hswap64(value: u64)->u64 {
        let other_bitmask=0x0000ffff0000ffff;
        value=value.rotate_left(32);
        return ((value & other_bitmask) << 16) | ((value >> 16) & other_bitmask);
    }
    pub fn wswap64(value: u64) {
        return value.rotate_left(32);
    }
}
