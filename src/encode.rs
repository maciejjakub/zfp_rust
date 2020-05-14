use num_traits::PrimInt;

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

// 1 Dimension
pub fn encode_cube1<I:PrimInt>(v: &mut [I; 4], p: usize) -> () {
    fwd_lift(v, p, 1);
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

#[cfg(test)]
mod tests {
    #[test]
    fn i8_test() {
        let mut v: [i8; 4] = [1, 2, 3, 4]; 
        super::encode_cube1(&mut v, 0);
        assert_eq!(v, [2, -1, 0, 0]);
    }
    #[test]
    fn i16_test() {
        let mut v: [i16; 4] = [1, 2, 3, 4]; 
        super::encode_cube1(&mut v, 0);
        assert_eq!(v, [2, -1, 0, 0]);
    }
    #[test]
    fn i32_test() {
        let mut v: [i32; 4] = [1, 2, 3, 4]; 
        super::encode_cube1(&mut v, 0);
        assert_eq!(v, [2, -1, 0, 0]);
    }
    #[test]
    fn i64_test() {
        let mut v: [i64; 4] = [1, 2, 3, 4]; 
        super::encode_cube1(&mut v, 0);
        assert_eq!(v, [2, -1, 0, 0]);
    }
    #[test]
    fn i128_test() {
        let mut v: [i128; 4] = [1, 2, 3, 4]; 
        super::encode_cube1(&mut v, 0);
        assert_eq!(v, [2, -1, 0, 0]);
    }
    #[test]
    fn test_2d() {
        let mut v: [i32; 16] = [67, 56, 43, 21, 67, 56, 43, 21, 67, 56, 43, 21, 67, 56, 43, 21]; 
        super::encode_cube2(&mut v, 0);
        assert_eq!(v, [46, 16, 3, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    }
    #[test]
    fn test_3d() {
        let mut v = vec![0, 11, 22, 33, 44, 55, 66, 77, 88, 99, 100, 111, 122, 133, 144, 155, 0, 11, 22, 33, 44, 55, 66, 77, 88, 99, 100, 111, 122, 133, 144, 155, 0, 11, 22, 33, 44, 55, 66, 77, 88, 99, 100, 111, 122, 133, 144, 155, 0, 11, 22, 33, 44, 55, 66, 77, 88, 99, 100, 111, 122, 133, 144, 155]; 
        super::encode_cube3(&mut v, 0);
        assert_eq!(v, vec![78, -10, 0, 0, -40, 0, 0, 1, 1, 1, 0, 1, 1, -2, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    }
}
