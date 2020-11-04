use std::ops::{Add, Shr, Shl, Sub};
pub trait LiftInt:
    Copy + Add<Self, Output = Self> + Sub<Self, Output = Self> + Shr<Self, Output = Self> + Shl<Self, Output = Self> + From<i8>
{
}
impl<
        T: Copy
            + Add<Self, Output = Self>
            + Sub<Self, Output = Self>
            + Shr<Self, Output = Self>
            + Shl<Self, Output = Self>
            + From<i8>,
    > LiftInt for T
{
}

fn inv_lift<I>(x: I, y: I, z: I, w: I) -> [I; 4]
where
    I: LiftInt,
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

// 1 Dimension
pub fn decode_cube1<I: LiftInt>(v: &mut [I; 4]) -> () {
    inv_lift(v[0], v[1], v[2], v[3])
        .iter()
        .enumerate()
        .for_each(|(e, val)| v[e] = *val);
}

// 2 Dimensions
pub fn decode_cube2<I: LiftInt>(v: &mut [I; 16]) -> () {
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
        inv_lift(
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
pub fn decode_cube3<I: LiftInt>(v: &mut [I; 64]) -> () {
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
        inv_lift(
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
        let mut v: [i8; 4] = [2, -1, 0, 0]; 
        super::decode_cube1(&mut v);
        assert_eq!(v, [0, 2, 2, 4]);
    }
    #[test]
    fn i16_test() {
        let mut v: [i16; 4] = [2, -1, 0, 0]; 
        super::decode_cube1(&mut v);
        assert_eq!(v, [0, 2, 2, 4]);
    }
    #[test]
    fn i32_test() {
        let mut v: [i32; 4] = [2, -1, 0, 0]; 
        super::decode_cube1(&mut v);
        assert_eq!(v, [0, 2, 2, 4]);
    }
    #[test]
    fn i64_test() {
        let mut v: [i64; 4] = [2, -1, 0, 0]; 
        super::decode_cube1(&mut v);
        assert_eq!(v, [0, 2, 2, 4]);
    }
    #[test]
    fn i128_test() {
        let mut v: [i128; 4] = [2, -1, 0, 0]; 
        super::decode_cube1(&mut v);
        assert_eq!(v, [0, 2, 2, 4]);
    }
    #[test]
    fn test_2d() {
        let mut v: [i32; 16] = [46, 16, 3, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]; 
        super::decode_cube2(&mut v);
        assert_eq!(v, [66, 56, 42, 20, 66, 56, 42, 20, 66, 56, 42, 20, 66, 56, 42, 20]);
    }
    #[test]
    fn test_3d() {
        let v_src: Vec<i32> = vec![
            78, -10, 0, 0, -40, 0, 0, 1, 1, 1, 0, 1, 1, -2, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1337, 1337, 1337, 1337,
        ];

        let mut v: [i32; 64] = [0; 64];
        v.copy_from_slice(&v_src[0..64]);
        super::decode_cube3(&mut v);

        assert_eq!(
            v.iter().map(|&x| x).collect::<Vec<i32>>(),
            vec![
                -1, 11, 21, 33, 43, 55, 65, 77, 87, 97, 99, 109, 122, 130, 146, 154,
                -1, 11, 21, 33, 43, 55, 65, 77, 87, 97, 99, 109, 122, 130, 146, 154, 
                -1, 11, 21, 33, 43, 55, 65, 77, 87, 97, 99, 109, 122, 130, 146, 154, 
                -1, 11, 21, 33, 43, 55, 65, 77, 87, 97, 99, 109, 122, 130, 146, 154
            ]
        );

        let v_should_be: [i32; 64] = [
            -1, 11, 21, 33, 43, 55, 65, 77, 87, 97, 99, 109, 122, 130, 146, 154,
            -1, 11, 21, 33, 43, 55, 65, 77, 87, 97, 99, 109, 122, 130, 146, 154, 
            -1, 11, 21, 33, 43, 55, 65, 77, 87, 97, 99, 109, 122, 130, 146, 154, 
            -1, 11, 21, 33, 43, 55, 65, 77, 87, 97, 99, 109, 122, 130, 146, 154,
        ];
        assert!(v_should_be.iter().zip(v.iter()).all(|(a, b)| a == b));
    }
}
