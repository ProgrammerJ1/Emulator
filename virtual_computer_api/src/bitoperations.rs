use std::mem::size_of;
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
    //see if bit is set
    pub fn test_bit(nr: u64,address:&u64)->bool {
        let p=unsafe{std::ptr::from_ref(address).offset((nr/((size_of::<u64>()*8) as u64)) as isize).as_ref()}.unwrap();
        let mut test=p.clone();
        test&=1<<(nr%((size_of::<u64>()*8) as u64));
        return test>0
    }
    //see test if bit is set and set bit
    pub fn test_and_set_bit(nr: u64,address:&mut u64)->bool {
        let p=unsafe{std::ptr::from_mut(address).offset((nr/((size_of::<u64>()*8) as u64)) as isize).as_mut()}.unwrap();
        let res=Self::test_bit(nr,address);
        Self::set_bit(nr%((size_of::<u64>()*8) as u64), p);
        return res;
    }
    //see test if bit is set and clear bit
    pub fn test_and_clear_bit(nr: u64,address:&mut u64)->bool {
        let p=unsafe{std::ptr::from_mut(address).offset((nr/((size_of::<u64>()*8) as u64)) as isize).as_mut()}.unwrap();
        let res=Self::test_bit(nr,address);
        Self::clear_bit(nr%((size_of::<u64>()*8) as u64), p);
        return res;
    }
    //see test if bit is set and change bit
    pub fn test_and_change_bit(nr: u64,address:&mut u64)->bool {
        let p=unsafe{std::ptr::from_mut(address).offset((nr/((size_of::<u64>()*8) as u64)) as isize).as_mut()}.unwrap();
        let res=Self::test_bit(nr,address);
        Self::change_bit(nr%((size_of::<u64>()*8) as u64), p);
        return res;
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
        if offset>=size {
            return size;
        }
        for nr in offset..size {
            if Self::test_bit(nr,address) {
                return nr;
            }
        }
        return size 
    }
    //find next cleared bit
    pub fn find_next_zero_bit(address:&u64,offset:u64,size:u64)->u64 {
        if offset>=size {
            return size;
        }
        for nr in offset..size {
            if !Self::test_bit(nr,address) {
                return nr;
            }
        }
        return size 
    }
    //find first set bit
    pub fn find_first_bit(address:&u64,offset:u64,size:u64)->u64 {
        if offset>=size {
            return size;
        }
        for nr in 0..size {
            if Self::test_bit(nr,address) {
                return nr;
            }
        }
        return size 
    }
    //find first cleared bit
    pub fn find_first_zero_bit(address:&u64,offset:u64,size:u64)->u64 {
        if offset>=size {
            return size;
        }
        for nr in 0..size {
            if !Self::test_bit(nr,address) {
                return nr;
            }
        }
        return size;
    }
    //rotate 8 bit value left
    pub fn rol8(word:u8,n:u32)->u8 {
        word.rotate_left(n)
    }
    //rotate 8 bit value right
    pub fn ror8(word:u8,n:u32)->u8 {
        word.rotate_right(n)
    }
    //rotate 16 bit value left
    pub fn rol16(word:u16,n:u32)->u16 {
        word.rotate_left(n)
    }
    //rotate 16 bit value right
    pub fn ror16(word:u16,n:u32)->u16 {
        word.rotate_right(n)
    }
    //rotate 32 bit value left
    pub fn rol32(word:u32,n:u32)->u32 {
        word.rotate_left(n)
    }
    //rotate 32 bit value right
    pub fn ror32(word:u32,n:u32)->u32 {
        word.rotate_right(n)
    }
    //rotate 64 bit value left
    pub fn rol64(word:u64,n:u32)->u64 {
        word.rotate_left(n)
    }
    //rotate 64 bit value right
    pub fn ror64(word:u64,n:u32)->u64 {
        word.rotate_right(n)
    }
    //swap 16 bit halfwords in a 32 bit word
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
    pub fn wswap64(value: u64)->u64 {
        return value.rotate_left(32);
    }
    //extract a value from a 32 bit number
    pub fn extract32(value:u32,start:u32,length:u32)->u32 {
        assert!(length>0&&length<=32);
        return (value>>start)&(2_u32.pow(32-length-1)&(2_u32.pow(length-1)-1))
    }
    //extract a value from a 8 bit number
    pub fn extract8(value:u8,start:u32,length:u32)->u8 {
        assert!(length>0&&length<=8);
        return (value>>start)&(2_u8.pow(8-length-1)&(2_u8.pow(length-1)-1))
    }
    //extract a value from a 16 bit number
    pub fn extract16(value:u16,start:u32,length:u32)->u16 {
        assert!(length>0&&length<=16);
        return (value>>start)&(2_u16.pow(16-length-1)&(2_u16.pow(length-1)-1))
    }
    //extract a value from a 64 bit number
    pub fn extract64(value:u64,start:u32,length:u32)->u64 {
        assert!(length>0&&length<=64);
        return (value>>start)&(2_u64.pow(64-length-1)&(2_u64.pow(length-1)-1));
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
            value&=~other_bitmask;
            value|=(field_value&other_bitmask);
        }
        return value;
    }
    //deposit bits of a 64 bit value into another.
    pub fn deposit64(mut value:u64,start:u32,length:u32,field_value:u32)->u64 {
        assert!(length>0&&length<=64);
        {
            let other_bitmask=(u64::MAX>>(64-length))<<start;
            value&=~other_bitmask;
            value|=(field_value&other_bitmask);
        }
        return value;
    }
    
    //deposit bits of a 16 bit value into another.
    pub fn deposit16(mut value:u16,start:u32,length:u32,field_value:u32)->u16 {
        assert!(length>0&&length<=16);
        {
            let other_bitmask=(u16::MAX>>(16-length))<<start;
            value&=~other_bitmask;
            value|=(field_value&other_bitmask);
        }
        return value;
    }
    //deposit bits of a 8 bit value into another.
    pub fn deposit8(mut value:u8,start:u32,length:u32,field_value:u32)->u8 {
        assert!(length>0&&length<=8);
        {
            let other_bitmask=(u8::MAX>>(8-length))<<start;
            value&=~other_bitmask;
            value|=(field_value&other_bitmask);
        }
        return value;
    }
    //
    pub fn half_shuffle32(mut value:u32)->u32 {
        let aligned_value_clone: u64=value.into();
        for index in (0..16) {
            if Self::test_bit()
    }
}
