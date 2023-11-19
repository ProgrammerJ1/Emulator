use virtual_computer_api;
use rand::{rngs::ThreadRng,thread_rng,RngCore};
#[test]
pub fn bit_operations() {
    let mut trng=thread_rng();
    bit_setting(&mut trng)
}
pub fn bit_setting(rng: &mut ThreadRng) {
    let mut numbers:[u64;2]=[rng.next_u64(),rng.next_u64()];
    virtual_computer_api::bitoperations::BitOperations::set_bit(32, &mut numbers[0]);
    virtual_computer_api::bitoperations::BitOperations::set_bit(68, &mut numbers[0]);
}