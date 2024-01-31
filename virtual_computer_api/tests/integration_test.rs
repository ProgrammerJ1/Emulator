use virtual_computer_api;
use rand::{rngs::ThreadRng, thread_rng, Rng};
#[test]
pub fn bits_operations() {
    let mut rng=thread_rng();
    bit_setting_test(&rng)
}
fn bit_setting_test(rng: &ThreadRng) {
    let data: [u64;16]=[rng.gen(),rng.gen(),rng.gen(),rng.gen()
    ,rng.gen(),rng.gen(),rng.gen(),rng.gen()
    ,rng.gen(),rng.gen(),rng.gen()
    ,rng.gen(),rng.gen(),rng.gen(),rng.gen(),rng.gen()];
}