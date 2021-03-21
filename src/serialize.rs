use bitvec::prelude::*;
// use std::ops::{Add, Shr, Sub};
// use netcdf;

pub trait SerializeTrait:
    From<u8> + bitvec::store::BitStore + bitvec::index::BitRegister
{
}
impl<
        T: From<u8>
            + bitvec::store::BitStore
            + bitvec::index::BitRegister,
    > SerializeTrait for T
{
}

pub fn bit_test(mut my_char: u8) -> () {
    let mut what_bit_am_i_testing = 0;
    while what_bit_am_i_testing < 8 {
        if (my_char & 0b1) != 0 {
            println!("bit {:?} is 1", what_bit_am_i_testing);
        }
        else {
            println!("bit {:?} is 0", what_bit_am_i_testing);
        }
        what_bit_am_i_testing += 1;
        my_char = my_char >> 1;
    }
}

pub fn test() {
    let a: u8 = 0b1111;
    let b: u8 = 0b1010;
    //println!("{:b}", a & b);
    let mut my_char = 0xaa;
    println!("{:b}", my_char & 0b01);
    println!("{:b}", my_char);
    my_char = my_char >> 1;
    println!("{:b}", my_char);
    my_char = my_char >> 1;
    println!("{:b}", my_char);
    my_char = my_char >> 1;
    println!("{:b}", my_char);
    my_char = my_char >> 1;
    println!("{:b}", my_char);
    my_char = my_char >> 1;
    println!("{:#08b}", 0b01);
    
}

pub fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

pub fn execute_in_main() -> () {
    let my_char: u8 = 0xaa;
    print_type_of(&my_char);
}

pub fn bitvec_test() -> () {
    let mut data = 0u8;
    println!("{:?}", data);
    let mut bits = data.view_bits_mut::<Msb0>();
    println!("{:?}", bits);
    bits.for_each(|idx, _bit| idx % 1 == 0);
    // assert_eq!(data, 0b100_100_10);
    println!("{:b}", bits);
    let mut bits = data.view_bits_mut::<Msb0>();
    println!("{:?}", bits[3]);
    bits.set(3, false);
    println!("{:?}", bits);
}

pub fn bit_transpose_vector<I: SerializeTrait>(arr: &[I]) -> () {
    let array_length = arr.len();
    let array_element_size = std::mem::size_of_val(&arr[0]) * 8;
    let mut i = 0;
    let mut bits_serialized = bitvec![Msb0, u8; 0; array_element_size * array_length];
    for x in 0..array_element_size {
        for y in 0..array_length {
            let bit_slice = BitSlice::<Msb0, _>::from_element(&arr[y])[x];
            bits_serialized.set(i, bit_slice);
            i = i + 1;
        }
    }
    println!("{:?}", bits_serialized);
    // bits_serialized
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn i8_test() {
//         let input: [u8; 4] = [0xaa, 0xbb, 0xcc, 0xdd];
//         let output = super::bit_transpose_vector(&input);
//         assert_eq!(input, output);
//     }
// }