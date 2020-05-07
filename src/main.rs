use num_traits::PrimInt;

fn main() {
    // let mut v: [i32; 16] = [67, 56, 43, 21, 67, 56, 43, 21, 67, 56, 43, 21, 67, 56, 43, 21];
    // let mut v: [i32; 4] = [67, 56, 43, 21];
    // let mut v: [i32; 16] = [0, 11, 22, 33, 44, 55, 66, 77, 88, 99, 100, 111, 122, 133, 144, 155];
    // let mut v = vec![0, 11, 22, 33, 44, 55, 66, 77, 88, 99, 100, 111, 122, 133, 144, 155, 0, 11, 22, 33, 44, 55, 66, 77, 88, 99, 100, 111, 122, 133, 144, 155, 0, 11, 22, 33, 44, 55, 66, 77, 88, 99, 100, 111, 122, 133, 144, 155, 0, 11, 22, 33, 44, 55, 66, 77, 88, 99, 100, 111, 122, 133, 144, 155];
    let mut v = vec![338, 277, 8, 95, 38, 47, 279, 349, 284, 484, 429, 125, 14, 326, 86, 409, 239, 438, 124, 198, 495, 220, 94, 243, 459, 148, 199, 322, 24, 346, 389, 81, 245, 4, 483, 95, 110, 345, 158, 335, 146, 476, 419, 284, 164, 318, 242, 179, 20, 367, 281, 119, 273, 465, 460, 175, 335, 201, 335, 43, 485, 321, 6, 458];

    println!("Array before transformation: {:?}", v);
    encode_cube3(&mut v, 0);
    println!("Array after forward lifting transform: {:?}", v);
    decode_cube3(&mut v, 0);
    println!("Array after inverse lifting transform: {:?}", v);

    // encode_cube2(&mut v, 0, 1);

    // let n :u8 = 0b01001100;
    // let result = n.signed_shl(1) + 1;
    // println!("{:b}", result);

}

fn encode_cube_test(v: &mut [i32; 4]) -> () {
    v[0] += v[3];
    v[0] >>= 1;
    v[3] -= v[0];

    v[2] += v[1];
    v[2] >>= 1;
    v[1] -= v[2];

    v[0] += v[2];
    v[0] >>= 1;
    v[2] -= v[0];

    v[3] += v[1];
    v[3] >>= 1;
    v[1] -= v[3];

    v[3] += v[1] >> 1;
    v[1] -= v[3] >> 1;
}

fn decode_cube_test(v: &mut [i32; 4]) -> () {
    v[1] += v[3] >> 1;
    v[3] -= v[1] >> 1;

    v[1] += v[3];
    v[3] <<= 1;
    v[3] -= v[1];

    v[2] += v[0];
    v[0] <<= 1;
    v[0] -= v[2];

    v[1] += v[2];
    v[2] <<= 1;
    v[2] -= v[1];

    v[3] += v[0];
    v[0] <<= 1;
    v[0] -= v[3];
}

fn fwd_lift<I:PrimInt>(v: &mut [I], mut p: usize, s: usize) -> () {
    let (mut x, mut y, mut z, mut w): (I, I, I, I);
    x = v[p]; p += s;
    y = v[p]; p += s;
    z = v[p]; p += s;
    w = v[p]; p += s;

    x = x + w;
    x = x >> 1;
    w = w - x;

    z = z + y;
    z = z >> 1;
    y = y - z;

    x = x + z;
    x = x >> 1;
    z = z - x;

    w = w + y;
    w = w >> 1;
    y = y - w;

    w = w + (y >> 1);
    y = y - (w >> 1);

    p -= s; v[p] = w;
    p -= s; v[p] = z;
    p -= s; v[p] = y;
    p -= s; v[p] = x;

}

fn inv_lift<I:PrimInt>(v: &mut [I], mut p: usize, s: usize) -> () {

    let (mut x, mut y, mut z, mut w): (I, I, I, I);
    x = v[p]; p += s;
    y = v[p]; p += s;
    z = v[p]; p += s;
    w = v[p]; p += s;

    

    y = y + (w >> 1);
    w = w - (y >> 1);

    y = y + w;
    w = w << 1;
    w = w - y;

    z = z + x;
    x = x << 1;
    x = x - z;

    y = y + z;
    z = z << 1;
    z = z - y;

    w = w + x;
    x = x << 1;
    x = x - w;

    p -= s; v[p] = w;
    p -= s; v[p] = z;
    p -= s; v[p] = y;
    p -= s; v[p] = x;
} 

// 1 Dimension
fn encode_cube1<I:PrimInt>(v: &mut [I; 4], p: usize) -> () {
    fwd_lift(v, p, 1);
}

fn decode_cube1<I:PrimInt>(v: &mut [I; 4], p: usize) -> () {
    inv_lift(v, p, 1);
}

// 2 Dimensions
fn encode_cube2<I:PrimInt>(v: &mut [I; 16], p: usize) -> () {
    // transform along x
    for y in 0..4 {
        fwd_lift(v, p + 4 * y, 1);
    }
    // transform along y
    for x in 0..4 {
        fwd_lift(v, p + 1 * x, 4);
    }
}

fn decode_cube2<I:PrimInt>(v: &mut [I; 16], p: usize) -> () {
    // let (mut x, mut y): (u8, u8);

    for x in 0..4 {
        inv_lift(v, p + 1 * x, 4);
    }
    for y in 0..4 {
        inv_lift(v, p + 4 * y, 1);
    }
}

// 3 Dimensions
fn encode_cube3<I:PrimInt>(v: &mut [I], p: usize) -> () {
    // transform along x
    for z in 0..4 {
        for y in 0..4 {
            fwd_lift(v, p + 4 * y + 16 * z, 1);
        }
    }
    // transform along y
    for x in 0..4 {
        for z in 0..4 {
            fwd_lift(v, p + 16 * z + 1 * x, 4);
        }
    }
    // transform along z
    for y in 0..4 {
        for x in 0..4 {
            fwd_lift(v, p + 1 * x + 4 * y, 16);
        }
    }
}

fn decode_cube3<I:PrimInt>(v: &mut [I], p: usize) -> () {
    for y in 0..4 {
        for x in 0..4 {
            inv_lift(v, p + 1 * x + 4 * y, 16);
        }
    }
    for x in 0..4 {
        for z in 0..4 {
            inv_lift(v, p + 16 * z + 1 * x, 4);
        }
    }
    for z in 0..4 {
        for y in 0..4 {
            inv_lift(v, p + 4 * y + 16 * z, 1);
        }
    }
    
    
}

