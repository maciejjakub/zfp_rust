use num_traits::PrimInt;

pub fn encode_cube_test(v: &mut [i32; 4]) -> () {
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

pub fn decode_cube_test(v: &mut [i32; 4]) -> () {
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

pub fn fwd_lift<I:PrimInt>(v: &mut [I], mut p: usize, s: usize) -> () {
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

pub fn inv_lift<I:PrimInt>(v: &mut [I], mut p: usize, s: usize) -> () {

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
pub fn encode_cube1<I:PrimInt>(v: &mut [I; 4], p: usize) -> () {
    fwd_lift(v, p, 1);
}

pub fn decode_cube1<I:PrimInt>(v: &mut [I; 4], p: usize) -> () {
    inv_lift(v, p, 1);
}

// 2 Dimensions
pub fn encode_cube2<I:PrimInt>(v: &mut [I; 16], p: usize) -> () {
    // transform along x
    for y in 0..4 {
        fwd_lift(v, p + 4 * y, 1);
    }
    // transform along y
    for x in 0..4 {
        fwd_lift(v, p + 1 * x, 4);
    }
}

pub fn decode_cube2<I:PrimInt>(v: &mut [I; 16], p: usize) -> () {
    // let (mut x, mut y): (u8, u8);

    for x in 0..4 {
        inv_lift(v, p + 1 * x, 4);
    }
    for y in 0..4 {
        inv_lift(v, p + 4 * y, 1);
    }
}

// 3 Dimensions
pub fn encode_cube3<I:PrimInt>(v: &mut [I], p: usize) -> () {
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

pub fn decode_cube3<I:PrimInt>(v: &mut [I], p: usize) -> () {
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
