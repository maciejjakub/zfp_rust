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

fn fwd_lift<I>(x: I, y: I, z: I, w: I) -> [I; 4]
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

// 1 Dimension
pub fn encode_cube1<I: LiftInt>(v: &mut [I; 4]) -> () {
    fwd_lift(v[0], v[1], v[2], v[3])
        .iter()
        .enumerate()
        .for_each(|(e, val)| v[e] = *val);
}

// 2 Dimensions
pub fn encode_cube2<I: LiftInt>(v: &mut [I; 16]) -> () {
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
        fwd_lift(
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

// 3 Dimensions
pub fn encode_cube3<I: LiftInt>(v: &mut [I; 64]) -> () {
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
        fwd_lift(
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

#[cfg(test)]
mod tests {
    #[test]
    fn i8_test() {
        let mut v: [i8; 4] = [1, 2, 3, 4];
        super::encode_cube1(&mut v);
        assert_eq!(v, [2, -1, 0, 0]);
    }
    #[test]
    fn i16_test() {
        let mut v: [i16; 4] = [1, 2, 3, 4];
        super::encode_cube1(&mut v);
        assert_eq!(v, [2, -1, 0, 0]);
    }
    #[test]
    fn i32_test() {
        let mut v: [i32; 4] = [1, 2, 3, 4];
        super::encode_cube1(&mut v);
        assert_eq!(v, [2, -1, 0, 0]);
    }
    #[test]
    fn i64_test() {
        let mut v: [i64; 4] = [1, 2, 3, 4];
        super::encode_cube1(&mut v);
        assert_eq!(v, [2, -1, 0, 0]);
    }
    #[test]
    fn i128_test() {
        let mut v: [i128; 4] = [1, 2, 3, 4];
        super::encode_cube1(&mut v);
        assert_eq!(v, [2, -1, 0, 0]);
    }
    #[test]
    fn test_2d() {
        let mut v: [i32; 16] = [
            67, 56, 43, 21, 67, 56, 43, 21, 67, 56, 43, 21, 67, 56, 43, 21,
        ];
        super::encode_cube2(&mut v);
        assert_eq!(v, [46, 16, 3, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    }
    #[test]
    fn test_3d() {
        // Direct, should work in principle, only comparison in assert was causing problems
        //let mut v: [i32; 64] = [
        //    0, 11, 22, 33, 44, 55, 66, 77, 88, 99, 100, 111, 122, 133, 144, 155, 0, 11, 22, 33, 44,
        //    55, 66, 77, 88, 99, 100, 111, 122, 133, 144, 155, 0, 11, 22, 33, 44, 55, 66, 77, 88,
        //    99, 100, 111, 122, 133, 144, 155, 0, 11, 22, 33, 44, 55, 66, 77, 88, 99, 100, 111, 122,
        //    133, 144, 155,
        //];

        //How will we do it in production; a vector with some additional 1337 padding on the right,
        // to simulate that we are only interested in a part of it
        let v_src: Vec<i32> = vec![
            0, 11, 22, 33, 44, 55, 66, 77, 88, 99, 100, 111, 122, 133, 144, 155, 
            0, 11, 22, 33, 44, 55, 66, 77, 88, 99, 100, 111, 122, 133, 144, 155, 
            0, 11, 22, 33, 44, 55, 66, 77, 88, 99, 100, 111, 122, 133, 144, 155, 
            0, 11, 22, 33, 44, 55, 66, 77, 88, 99, 100, 111, 122, 133, 144, 155, 
            1337, 1337, 1337, 1337,
        ];
        //Fresh memory, zero-initialised; we don't want to zfp on the original version of the data,
        //because zfp will destroy it. In real code, this will be initialised once for many chunks,
        //so the cost will average out. Also, LLVM will probably figure out we overwrite these
        //zeroes right away, and remove the initialisation code.
        let mut v: [i32; 64] = [0; 64];
        //We do rewrite v with contents of an (arbitrary in general) slice of v_src
        v.copy_from_slice(&v_src[0..64]);

        super::encode_cube3(&mut v);
        //Here we convert array to vec for a sole act of comparisons with a derived code
        assert_eq!(
            v.iter().map(|&x| x).collect::<Vec<i32>>(),
            vec![
                78, -10, 0, 0, -40, 0, 0, 1, 1, 1, 0, 1, 1, -2, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
            ]
        );
        //Another way would be to implement array comparison, i.e.
        let v_should_be: [i32; 64] = [
            78, -10, 0, 0, -40, 0, 0, 1, 1, 1, 0, 1, 1, -2, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
        ];
        assert!(v_should_be.iter().zip(v.iter()).all(|(a, b)| a == b));
    }
}
