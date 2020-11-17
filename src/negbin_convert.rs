
pub trait Int2Uint<T> {
    fn int2uint(x: Self) -> T;
}
impl Int2Uint<u64> for i64 {
    fn int2uint(x: i64) -> u64 {
        let mask = 0xaaaa_aaaa_aaaa_aaaa;
        ((x as u64) + mask) ^ mask
    }
}
impl Int2Uint<u32> for i32 {
    fn int2uint(x: i32) -> u32 {
        let mask = 0xaaaa_aaaa;
        ((x as u32) + mask) ^ mask
    }
}
impl Int2Uint<u16> for i16 {
    fn int2uint(x: i16) -> u16 {
        let mask = 0xaaaa;
        ((x as u16) + mask) ^ mask
    }
}
impl Int2Uint<u8> for i8 {
    fn int2uint(x: i8) -> u8 {
        let mask = 0xaa;
        ((x as u8) + mask) ^ mask
    }
}

pub trait Uint2Int<T> {
    fn uint2int(x: Self) -> T;
}
impl Uint2Int<i64> for u64 {
    fn uint2int(x: u64) -> i64 {
        let mask = 0xaaaa_aaaa_aaaa_aaaa;
        ((x ^ mask) - mask) as i64
    }
}
impl Uint2Int<i32> for u32 {
    fn uint2int(x: u32) -> i32 {
        let mask = 0xaaaa_aaaa;
        ((x ^ mask) - mask) as i32
    }
}
impl Uint2Int<i16> for u16 {
    fn uint2int(x: u16) -> i16 {
        let mask = 0xaaaa;
        ((x ^ mask) - mask) as i16
    }
}
impl Uint2Int<i8> for u8 {
    fn uint2int(x: u8) -> i8 {
        let mask = 0xaa;
        ((x ^ mask) - mask) as i8
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn i64_test() {
        let input: i64 = 2362376134136134513;
        let output = super::Uint2Int::uint2int(super::Int2Uint::int2uint(input));
        assert_eq!(input, output);
    }
    #[test]
    fn i32_test() {
        let input: i32 = 21372137;
        let output = super::Uint2Int::uint2int(super::Int2Uint::int2uint(input));
        assert_eq!(input, output);
    }
    #[test]
    fn i16_test() {
        let input: i16 = 2137;
        let output = super::Uint2Int::uint2int(super::Int2Uint::int2uint(input));
        assert_eq!(input, output);
    }
    #[test]
    fn i8_test() {
        let input: i8 = 120;
        let output = super::Uint2Int::uint2int(super::Int2Uint::int2uint(input));
        assert_eq!(input, output);
    }
}
