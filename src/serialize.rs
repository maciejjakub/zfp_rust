use bitvec::prelude::*;

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

pub fn bit_transpose_vector() -> () {
    let my_arr: [u8; 4] = [0xaa, 0xbb, 0xcc, 0xdd];
    let array_length = my_arr.len();
    let array_element_size = std::mem::size_of_val(&my_arr[0]) * 8;
    let bit_slice_first_el_of_array = BitSlice::<Msb0, _>::from_element(&my_arr[0]);
    let mut i = 0;
    let mut bits_serialized = bitvec![Msb0, u8; 0; 32];
    for x in 0..array_element_size {
        for y in 0..array_length {
            let bit_slice = BitSlice::<Msb0, _>::from_element(&my_arr[y])[x];
            println!("{:?}", bit_slice);
            // bits_serialized[i] = bit_slice;
            i += i;
            // println!("{:?}", bit_slice_first_el_of_array[x]);
        }
    }
    println!("{:?}", my_arr.len());
    println!("{:?}", bits_serialized);
    println!("{:?}", bits_serialized[4]);
    // bits_serialized[4] = true;
}

pub fn bitvec_test() -> () {
    let mut data = 0u8;
    println!("{:?}", data);
    let bits = data.view_bits_mut::<Msb0>();
    println!("{:?}", bits);
    bits.for_each(|idx, _bit| idx % 3 == 0);
    // assert_eq!(data, 0b100_100_10);
}