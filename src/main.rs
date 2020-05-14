use zfp_rust;

fn main() {
    // let mut v: [i32; 16] = [67, 56, 43, 21, 67, 56, 43, 21, 67, 56, 43, 21, 67, 56, 43, 21];
    // let mut v: [i32; 4] = [67, 56, 43, 21];
    // let mut v: [i8; 4] = [1, 2, 3, 4];
    // let mut v: [i32; 16] = [0, 11, 22, 33, 44, 55, 66, 77, 88, 99, 100, 111, 122, 133, 144, 155];
    let mut v = vec![0, 11, 22, 33, 44, 55, 66, 77, 88, 99, 100, 111, 122, 133, 144, 155, 0, 11, 22, 33, 44, 55, 66, 77, 88, 99, 100, 111, 122, 133, 144, 155, 0, 11, 22, 33, 44, 55, 66, 77, 88, 99, 100, 111, 122, 133, 144, 155, 0, 11, 22, 33, 44, 55, 66, 77, 88, 99, 100, 111, 122, 133, 144, 155];
    // let mut v = vec![338, 277, 8, 95, 38, 47, 279, 349, 284, 484, 429, 125, 14, 326, 86, 409, 239, 438, 124, 198, 495, 220, 94, 243, 459, 148, 199, 322, 24, 346, 389, 81, 245, 4, 483, 95, 110, 345, 158, 335, 146, 476, 419, 284, 164, 318, 242, 179, 20, 367, 281, 119, 273, 465, 460, 175, 335, 201, 335, 43, 485, 321, 6, 458];

    println!("Array before transformation: {:?}", v);
    zfp_rust::encode::encode_cube3(&mut v, 0);
    println!("Array after forward lifting transform: {:?}", v);
    zfp_rust::decode::decode_cube3(&mut v, 0);
    println!("Array after inverse lifting transform: {:?}", v);
}
