use num_traits::PrimInt;

use std::ops::{Add, Shr, Shl, Sub};
pub trait LiftUint:
    Copy + Add<Self, Output = Self> + Sub<Self, Output = Self> + Shr<Self, Output = Self> + Shl<Self, Output = Self> + From<u8>
{
}
impl<
        T: Copy
            + Add<Self, Output = Self>
            + Sub<Self, Output = Self>
            + Shr<Self, Output = Self>
            + Shl<Self, Output = Self>
            + From<u8>,
    > LiftUint for T
{
}

pub trait NegbinConvert<T> {
    fn negbin_convert(x: Self) -> T;
}
impl NegbinConvert<i64> for u64 {
    fn negbin_convert(x: u64) -> i64 {
        let mask = 0xaaaa_aaaa_aaaa_aaaa;
        ((x ^ mask) - mask) as i64
    }
}
impl NegbinConvert<i32> for u32 {
    fn negbin_convert(x: u32) -> i32 {
        let mask = 0xaaaa_aaaa;
        ((x ^ mask) - mask) as i32
    }
}
impl NegbinConvert<i16> for u16 {
    fn negbin_convert(x: u16) -> i16 {
        let mask = 0xaaaa;
        ((x ^ mask) - mask) as i16
    }
}
impl NegbinConvert<i8> for u8 {
    fn negbin_convert(x: u8) -> i8 {
        let mask = 0xaa;
        ((x ^ mask) - mask) as i8
    }
}

fn inv_lift_alt2<I>(x: I, y: I, z: I, w: I) -> [I; 4]
where
    I: LiftUint,
{
    let one = I::from(1);
    let mut y = y + (w >> one);
    let mut w = w - (y >> one);

    y = y + w;
    w = w << one;
    w = w - y;

    let mut z = z + x;
    let mut x = x << one;
    x = x - z;

    y = y + z;
    z = z << one;
    z = z - y;

    w = w + x;
    x = x << one;
    x = x - w;

    [x, y, z, w]
}

pub fn decode_cube1_alt3<I: LiftUint>(v: &mut [I; 4]) -> () {
    inv_lift_alt2(v[0], v[1], v[2], v[3])
        .iter()
        .enumerate()
        .for_each(|(e, val)| v[e] = *val);
}

pub fn decode_cube2_alt3<I: LiftUint>(v: &mut [I; 16]) -> () {
    //D is decides how far into todo we go
    let todo = [
        (0, 4),
        (1, 4),
        (2, 4),
        (3, 4),
        (0, 1),
        (4, 1),
        (8, 1),
        (12, 1),
    ];
    for (offset, hop) in todo.iter() {
        inv_lift_alt2(
            v[offset + 0],
            v[offset + hop],
            v[offset + hop * 2],
            v[offset + hop * 3],
        )
        .iter()
        .enumerate()
        .for_each(|(e, val)| v[offset + hop * e] = *val);
    }
}

pub fn decode_cube3_alt3<I: LiftUint>(v: &mut [I; 64]) -> () {
    //D is decides how far into todo we go
    let todo = [
        (0, 16),
        (1, 16),
        (2, 16),
        (3, 16),
        (4, 16),
        (5, 16),
        (6, 16),
        (7, 16),
        (8, 16),
        (9, 16),
        (10, 16),
        (11, 16),
        (12, 16),
        (13, 16),
        (14, 16),
        (15, 16),
        (0, 4),
        (16, 4),
        (32, 4),
        (48, 4),
        (1, 4),
        (17, 4),
        (33, 4),
        (49, 4),
        (2, 4),
        (18, 4),
        (34, 4),
        (50, 4),
        (3, 4),
        (19, 4),
        (35, 4),
        (51, 4),
        (0, 1),
        (4, 1),
        (8, 1),
        (12, 1),
        (16, 1),
        (20, 1),
        (24, 1),
        (28, 1),
        (32, 1),
        (36, 1),
        (40, 1),
        (44, 1),
        (48, 1),
        (52, 1),
        (56, 1),
        (60, 1),
    ];
    for (offset, hop) in todo.iter() {
        inv_lift_alt2(
            v[offset + 0],
            v[offset + hop],
            v[offset + hop * 2],
            v[offset + hop * 3],
        )
        .iter()
        .enumerate()
        .for_each(|(e, val)| v[offset + hop * e] = *val);
    }
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
pub fn decode_cube1<I:PrimInt>(v: &mut [I; 4], p: usize) -> () {
    inv_lift(v, p, 1);
}

// 2 Dimensions
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
    
#[cfg(test)]
mod tests {
    #[test]
    fn i8_test() {
        let mut v: [i8; 4] = [2, -1, 0, 0]; 
        super::decode_cube1(&mut v, 0);
        assert_eq!(v, [0, 2, 2, 4]);
    }
    #[test]
    fn i16_test() {
        let mut v: [i16; 4] = [2, -1, 0, 0]; 
        super::decode_cube1(&mut v, 0);
        assert_eq!(v, [0, 2, 2, 4]);
    }
    #[test]
    fn i32_test() {
        let mut v: [i32; 4] = [2, -1, 0, 0]; 
        super::decode_cube1(&mut v, 0);
        assert_eq!(v, [0, 2, 2, 4]);
    }
    #[test]
    fn i64_test() {
        let mut v: [i64; 4] = [2, -1, 0, 0]; 
        super::decode_cube1(&mut v, 0);
        assert_eq!(v, [0, 2, 2, 4]);
    }
    #[test]
    fn i128_test() {
        let mut v: [i128; 4] = [2, -1, 0, 0]; 
        super::decode_cube1(&mut v, 0);
        assert_eq!(v, [0, 2, 2, 4]);
    }
    #[test]
    fn test_2d() {
        let mut v: [i32; 16] = [46, 16, 3, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]; 
        super::decode_cube2(&mut v, 0);
        assert_eq!(v, [66, 56, 42, 20, 66, 56, 42, 20, 66, 56, 42, 20, 66, 56, 42, 20]);
    }
    #[test]
    fn test_3d() {
        let mut v = vec![78, -10, 0, 0, -40, 0, 0, 1, 1, 1, 0, 1, 1, -2, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]; 
        super::decode_cube3(&mut v, 0);
        assert_eq!(v, vec![-1, 11, 21, 33, 43, 55, 65, 77, 87, 97, 99, 109, 122, 130, 146, 154, -1, 11, 21, 33, 43, 55, 65, 77, 87, 97, 99, 109, 122, 130, 146, 154, -1, 11, 21, 33, 43, 55, 65, 77, 87, 97, 99, 109, 122, 130, 146, 154, -1, 11, 21, 33, 43, 55, 65, 77, 87, 97, 99, 109, 122, 130, 146, 154]);
    }
}