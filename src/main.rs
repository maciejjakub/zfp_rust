use zfp_rust;
use bitvec::prelude::*;

fn main() {
    // let mut v: [i32; 16] = [67, 56, 43, 21, 67, 56, 43, 21, 67, 56, 43, 21, 67, 56, 43, 21];
    let mut v: [i64; 4] = [67, 56, 43, 21];
    // let mut v: [i16; 4] = [1, 2, 3, 4];
    // let mut v: [i32; 16] = [0, 11, 22, 33, 44, 55, 66, 77, 88, 99, 100, 111, 122, 133, 144, 155];
    // let mut v = vec![0, 11, 22, 33, 44, 55, 66, 77, 88, 99, 100, 111, 122, 133, 144, 155, 0, 11, 22, 33, 44, 55, 66, 77, 88, 99, 100, 111, 122, 133, 144, 155, 0, 11, 22, 33, 44, 55, 66, 77, 88, 99, 100, 111, 122, 133, 144, 155, 0, 11, 22, 33, 44, 55, 66, 77, 88, 99, 100, 111, 122, 133, 144, 155];
    // let mut v: [i32; 64] = [0, 11, 22, 33, 44, 55, 66, 77, 88, 99, 100, 111, 122, 133, 144, 155, 0, 11, 22, 33, 44, 55, 66, 77, 88, 99, 100, 111, 122, 133, 144, 155, 0, 11, 22, 33, 44, 55, 66, 77, 88, 99, 100, 111, 122, 133, 144, 155, 0, 11, 22, 33, 44, 55, 66, 77, 88, 99, 100, 111, 122, 133, 144, 155];
    // let mut v = vec![338, 277, 8, 95, 38, 47, 279, 349, 284, 484, 429, 125, 14, 326, 86, 409, 239, 438, 124, 198, 495, 220, 94, 243, 459, 148, 199, 322, 24, 346, 389, 81, 245, 4, 483, 95, 110, 345, 158, 335, 146, 476, 419, 284, 164, 318, 242, 179, 20, 367, 281, 119, 273, 465, 460, 175, 335, 201, 335, 43, 485, 321, 6, 458];

    println!("Array before transformation: {:?}", v);
    zfp_rust::encode::encode_cube1(&mut v);
    println!("Array after forward lifting transform: {:?}", v);
    zfp_rust::decode::decode_cube1(&mut v);
    println!("Array after inverse lifting transform: {:?}", v);
    let a: i16 = -45;
    let b: u32 = 0b1111111111001110;
    println!("Wynik: {:#b}", a);
    println!("Wynik: {:?}", b);

    let x: i16 = 0xaa;
    let y: i16 = 45;

    let a = zfp_rust::negbin_convert::Int2Uint::int2uint(x);
    println!("{:?}", x);
    println!("{:?}", a);
    let b = zfp_rust::negbin_convert::Uint2Int::uint2int(a);
    println!("{:?}", b);

    println!("{:?}", "------------------------");

    let v: [i32; 16] = [0, 11, 22, 33, 44, 55, 66, 77, 88, 99, 100, 111, 122, 133, 144, 155];
    let output = zfp_rust::fwd_order::fwd_order2(v);
    println!("{:?}", output);
    let output = zfp_rust::inv_order::inv_order2(output);
    println!("{:?}", output);
    // let arrr = [3, 3, 3, 3];
    // zfp_rust::codec1::cache_align(arrr);
    let my_char = 0xaa;
    zfp_rust::serialize::bit_test(my_char);

    let arr = BitSlice::<Msb0, _>::from_element(&my_char);
    println!("{:?}", arr);
}
