use num_traits::PrimInt;

use std::ops::{Add, Shr, Sub};
pub trait LiftInt:
    Copy + Add<Self, Output = Self> + Sub<Self, Output = Self> + Shr<Self, Output = Self> + From<i8>
{
}
impl<
        T: Copy
            + Add<Self, Output = Self>
            + Sub<Self, Output = Self>
            + Shr<Self, Output = Self>
            + From<i8>,
    > LiftInt for T
{
}

pub trait NegbinConvert<T> {
    fn negbin_convert(x: Self) -> T;
}
impl NegbinConvert<u64> for i64 {
    fn negbin_convert(x: i64) -> u64 {
        let mask = 0xaaaa_aaaa_aaaa_aaaa;
        ((x as u64) + mask) ^ mask
    }
}
impl NegbinConvert<u32> for i32 {
    fn negbin_convert(x: i32) -> u32 {
        let mask = 0xaaaa_aaaa;
        ((x as u32) + mask) ^ mask
    }
}
impl NegbinConvert<u16> for i16 {
    fn negbin_convert(x: i16) -> u16 {
        let mask = 0xaaaa;
        ((x as u16) + mask) ^ mask
    }
}
impl NegbinConvert<u8> for i8 {
    fn negbin_convert(x: i8) -> u8 {
        let mask = 0xaa;
        ((x as u8) + mask) ^ mask
    }
}

fn fwd_lift_alt<I>(x: I, y: I, z: I, w: I) -> (I, I, I, I)
where
    I: LiftInt,
{
    let one = I::from(1);
    let mut x = x + w;
    x = x >> one;
    let w = w - x;

    let mut z = z + y;
    z = z >> one;
    let mut y = y - z;

    x = x + z;
    x = x >> one;
    z = z - x;

    let mut w = w + y;
    w = w >> one;
    y = y - w;

    w = w + (y >> one);
    y = y - (w >> one);

    (x, y, z, w)
}

pub fn encode_cube1_alt<I: LiftInt>(v: &mut [I; 4]) -> () {
    let ans = fwd_lift_alt(v[0], v[1], v[2], v[3]);
    v[0] = ans.0;
    v[1] = ans.1;
    v[2] = ans.2;
    v[3] = ans.3;
}

fn fwd_lift_alt2<I>(x: I, y: I, z: I, w: I) -> [I; 4]
where
    I: LiftInt,
{
    let one = I::from(1);
    let mut x = x + w;
    x = x >> one;
    let w = w - x;

    let mut z = z + y;
    z = z >> one;
    let mut y = y - z;

    x = x + z;
    x = x >> one;
    z = z - x;

    let mut w = w + y;
    w = w >> one;
    y = y - w;

    w = w + (y >> one);
    y = y - (w >> one);

    [x, y, z, w]
}

pub fn encode_cube1_alt2<I: LiftInt>(v: &mut [I; 4]) -> () {
    *v = fwd_lift_alt2(v[0], v[1], v[2], v[3]);
}

pub fn encode_cube1_alt3<I: LiftInt>(v: &mut [I; 4]) -> () {
    fwd_lift_alt2(v[0], v[1], v[2], v[3])
        .iter()
        .enumerate()
        .for_each(|(e, val)| v[e] = *val);
}

pub fn encode_cube2_alt3<I: LiftInt>(v: &mut [I; 16]) -> () {
    //D is decides how far into todo we go
    let todo = [
        (0, 1),
        (4, 1),
        (8, 1),
        (12, 1),
        (0, 4),
        (1, 4),
        (2, 4),
        (3, 4),
    ];
    for (offset, hop) in todo.iter() {
        fwd_lift_alt2(
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

pub fn encode_cube3_alt3<I: LiftInt>(v: &mut [I; 64]) -> () {
    //D is decides how far into todo we go
    let todo = [
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
    ];
    for (offset, hop) in todo.iter() {
        fwd_lift_alt2(
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

pub fn fwd_lift<I: PrimInt>(v: &mut [I], mut p: usize, s: usize) -> () {
    let (mut x, mut y, mut z, mut w): (I, I, I, I);
    x = v[p];
    p += s;
    y = v[p];
    p += s;
    z = v[p];
    p += s;
    w = v[p];
    p += s; //No-op with next p-=s

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

    p -= s; //No-op with last p+=s
    v[p] = w;
    p -= s;
    v[p] = z;
    p -= s;
    v[p] = y;
    p -= s;
    v[p] = x;
}

// 1 Dimension
pub fn encode_cube1<I: PrimInt>(v: &mut [I; 4], p: usize) -> () {
    fwd_lift(v, p, 1);
}

// 2 Dimensions
pub fn encode_cube2<I: PrimInt>(v: &mut [I; 16], p: usize) -> () {
    // transform along x
    for y in 0..4 {
        fwd_lift(v, p + 4 * y, 1);
    }
    // transform along y
    for x in 0..4 {
        fwd_lift(v, p + 1 * x, 4);
    }
}

// 3 Dimensions
pub fn encode_cube3<I: PrimInt>(v: &mut [I], p: usize) -> () {
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

#[cfg(test)]
mod tests {
    #[test]
    fn i32_test_alt() {
        let mut v: [i32; 4] = [1, 2, 3, 4];
        super::encode_cube1_alt(&mut v);
        assert_eq!(v, [2, -1, 0, 0]);
    }
    //Lift makes sense on unsigned only
    #[test]
    fn i8_test() {
        let mut v: [i8; 4] = [1, 2, 3, 4];
        super::encode_cube1_alt3(&mut v);
        assert_eq!(v, [2, -1, 0, 0]);
    }
    #[test]
    fn i16_test() {
        let mut v: [i16; 4] = [1, 2, 3, 4];
        super::encode_cube1_alt3(&mut v);
        assert_eq!(v, [2, -1, 0, 0]);
    }
    #[test]
    fn i32_test() {
        let mut v: [i32; 4] = [1, 2, 3, 4];
        super::encode_cube1_alt3(&mut v);
        assert_eq!(v, [2, -1, 0, 0]);
    }
    #[test]
    fn i64_test() {
        let mut v: [i64; 4] = [1, 2, 3, 4];
        super::encode_cube1_alt3(&mut v);
        assert_eq!(v, [2, -1, 0, 0]);
    }
    #[test]
    fn i128_test() {
        let mut v: [i128; 4] = [1, 2, 3, 4];
        super::encode_cube1_alt3(&mut v);
        assert_eq!(v, [2, -1, 0, 0]);
    }
    #[test]
    fn test_2d() {
        let mut v: [i32; 16] = [
            67, 56, 43, 21, 67, 56, 43, 21, 67, 56, 43, 21, 67, 56, 43, 21,
        ];
        super::encode_cube2_alt3(&mut v);
        assert_eq!(v, [46, 16, 3, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    }
    #[test]
    fn test_3d() {
        let mut v = vec![
            0, 11, 22, 33, 44, 55, 66, 77, 88, 99, 100, 111, 122, 133, 144, 155, 0, 11, 22, 33, 44,
            55, 66, 77, 88, 99, 100, 111, 122, 133, 144, 155, 0, 11, 22, 33, 44, 55, 66, 77, 88,
            99, 100, 111, 122, 133, 144, 155, 0, 11, 22, 33, 44, 55, 66, 77, 88, 99, 100, 111, 122,
            133, 144, 155,
        ];
        super::encode_cube3_alt3(&mut v);
        assert_eq!(
            v,
            vec![
                78, -10, 0, 0, -40, 0, 0, 1, 1, 1, 0, 1, 1, -2, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
            ]
        );
    }
}
