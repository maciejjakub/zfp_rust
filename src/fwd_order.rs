use std::fmt::Debug;

pub trait OrderInt:
    Copy + From<i8> + Debug
{
}
impl<
        T: Copy
        	+ Debug
            + From<i8>,
    > OrderInt for T
{
}

pub fn fwd_order1<I: OrderInt>(input_list: [I; 4]) -> [I; 4] {
	let perm_table = [0, 1, 2, 3];
	let mut output_list: [I; 4] = [I::from(0); 4];
	for i in 0..4 {
		output_list[i] = input_list[perm_table[i]];
	}
	output_list
}

pub fn fwd_order2<I: OrderInt>(input_list: [I; 16]) -> [I; 16] {
	let perm_table = [0, 1, 4, 5, 2, 8, 6, 9, 3, 12, 10, 7, 13, 11, 14, 15];
	let mut output_list: [I; 16] = [I::from(0); 16];
	for i in 0..16 {
		output_list[i] = input_list[perm_table[i]];
	}
	output_list
}

pub fn fwd_order3<I: OrderInt>(input_list: [I; 64]) -> [I; 64] {
	let perm_table = [0, 1, 4, 4, 8, 5, 5, 2, 8, 8, 9, 6, 6, 12, 9, 9, 12, 3, 12, 12, 10, 13, 13, 16, 10, 10, 7, 7, 16, 13, 13, 16, 17, 14, 14, 11, 17, 17, 11, 11, 20, 14, 14, 20, 18, 15, 15, 21, 18, 18, 21, 24, 15, 15, 19, 22, 22, 25, 19, 19, 26, 23, 23, 27];
	let mut output_list: [I; 64] = [I::from(0); 64];
	for i in 0..64 {
		output_list[i] = input_list[perm_table[i]];
	}
	output_list
}

#[cfg(test)]
mod tests {
    #[test]
    fn i8_test() {
        let input: [i8; 4] = [1, 2, 3, 4];
        let output = super::fwd_order1(input);
        assert_eq!(output, [1, 2, 3, 4]);
    }
    #[test]
    fn i16_test() {
        let input: [i16; 4] = [1, 2, 3, 4];
        let output = super::fwd_order1(input);
        assert_eq!(output, [1, 2, 3, 4]);
    }
    #[test]
    fn i32_test() {
        let input: [i32; 4] = [1, 2, 3, 4];
        let output = super::fwd_order1(input);
        assert_eq!(output, [1, 2, 3, 4]);
    }
    #[test]
    fn i64_test() {
        let input: [i64; 4] = [1, 2, 3, 4];
        let output = super::fwd_order1(input);
        assert_eq!(output, [1, 2, 3, 4]);
    }
    #[test]
    fn i32_test2d() {
        let input: [i32; 16] = [0, 11, 22, 33, 44, 55, 66, 77, 88, 99, 100, 111, 122, 133, 144, 155];
        let output = super::fwd_order2(input);
        assert_eq!(output, [0, 11, 44, 55, 22, 88, 66, 99, 33, 122, 100, 77, 133, 111, 144, 155]);
    }
    #[test]
    fn i32_test3d() {
        let input: [i32; 64] = [
        	169, 556, 671, 952, 349, 802, 5, 187, 985, 722, 264, 556, 811, 444,
        	246, 37, 919, 788, 800, 765, 689, 567, 944, 499, 487, 428, 98, 774,
        	623, 620, 454, 859, 12, 752, 734, 984, 707, 435, 489, 810, 829, 631, 
        	623, 600, 194, 36, 231, 315, 329, 747, 335, 138, 704, 251, 464, 346, 
        	376, 983, 993, 130, 579, 791, 923, 486
        	];
        let output = super::fwd_order3(input);
        let output_should_be: [i32; 64] = [
        	169, 556, 349, 349, 985, 802, 802, 671, 985, 985, 722, 5, 5, 811, 722, 
        	722, 811, 952, 811, 811, 264, 444, 444, 919, 264, 264, 187, 187, 919, 
        	444, 444, 919, 788, 246, 246, 556, 788, 788, 556, 556, 689, 246, 246, 
        	689, 800, 37, 37, 567, 800, 800, 567, 487, 37, 37, 765, 944, 944, 428,
        	765, 765, 98, 499, 499, 774
        	];
        assert!(output_should_be.iter().zip(output.iter()).all(|(a, b)| a == b));
    }
}

