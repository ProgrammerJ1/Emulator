use virtual_computer_api;
use rand::{rngs::ThreadRng,thread_rng,RngCore};
#[test]
pub fn bit_operations() {
    let mut trng=thread_rng();
    bit_setting(&mut trng)
}
pub fn bit_setting(rng: &mut ThreadRng) {
    let mut numbers:[u64;3]=[rng.next_u64(),rng.next_u64(),rng.next_u64()];
    {
        for index in 0..3 {
            if numbers[index]==u64::MAX {
                while numbers[index]==u64::MAX {
                    numbers[index]=rng.next_u64();
                }
            }
        }
    }
    let modified_bits:[u32; 3];
    let mut new_number_strings:[String; 3];
    {
        let old_number_strings:[String; 3]=[numbers[0].to_string(),numbers[1].to_string(),numbers[2].to_string()];
        modified_bits=[63-unsafe{old_number_strings[0].rfind('0').unwrap_unchecked()} as u64,63+(63-unsafe{old_number_strings[1].rfind('0').unwrap_unchecked()}) as u64,63-unsafe{old_number_strings[0].find('1').unwrap_unchecked()} as u64];
        new_number_strings=old_number_strings.clone();
        for i in 0..2 {
            let chosen_bit=new_number_strings[i].rmatch_indices('0').next().unwrap_unchecked().0];
            *new_number_strings[i][chosen_bit..chosen_bit]=*"1";
        }
    }
    virtual_computer_api::bitoperations::BitOperations::set_bit(modified_bits[0], &mut numbers[0]);
    virtual_computer_api::bitoperations::BitOperations::set_bit(modified_bits[1], &mut numbers[0]);
    virtual_computer_api::bitoperations::BitOperations::set_bit(modified_bits[2], &mut numbers[2]);
    for i in 0..3 {
        //Assertations will go here
    }
}
