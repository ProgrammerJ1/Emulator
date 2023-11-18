use virtual_computer_api;
use rand::{rngs::ThreadRng,thread_rng};
#[test]
pub fn bit_operations() {
    let mut trng=thread_rng();;
    bit_setting(&mut trng)
}
pub fn bit_setting(rng: &mut ThreadRng) {
    let mut numbers:[u64;2]=[rng.next_u64(),rng.next_u64()];
    numbers=[]
}