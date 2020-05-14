use num_traits::PrimInt;

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