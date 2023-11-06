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
    pub fn test_and_change_bit(nr: u64,address:&mut u64)->bool {
        let p=unsafe{std::ptr::from_mut(address).offset((nr/((size_of::<u64>()*8) as u64)) as isize).as_mut()}.unwrap();
        let mut test=p.clone();
        test&=1<<(nr%((size_of::<u64>()*8) as u64));
        Self::change_bit(nr%((size_of::<u64>()*8) as u64), p);
        return test>0
    }
    
    pub fn test_bit(nr: u64,address:&mut u64)->bool {
        let p=unsafe{std::ptr::from_mut(address).offset((nr/((size_of::<u64>()*8) as u64)) as isize).as_mut()}.unwrap();
        let mut test=p.clone();
        test&=1<<(nr%((size_of::<u64>()*8) as u64));
        return test>0
    }
}