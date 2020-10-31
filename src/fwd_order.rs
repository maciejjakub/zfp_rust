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
		println!("{:?}", input_list[perm_table[i]]);
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

