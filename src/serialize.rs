pub fn bit_test(mut my_char: u8) -> () {
	let mut what_bit_am_i_testing = 0;
	while what_bit_am_i_testing < 8 {
		if (my_char & 0b1) != 0 {
			println!("bit {:?} is 1", what_bit_am_i_testing);
		}
		else {
			println!("bit {:?} is 0", what_bit_am_i_testing);
		}
		what_bit_am_i_testing += 1;
		my_char = my_char >> 1;
	}
}

pub fn test() {
    let a: u8 = 0b1111;
    let b: u8 = 0b1010;
    //println!("{:b}", a & b);
    let mut my_char = 0xaa;
    println!("{:b}", my_char & 0b01);
    println!("{:b}", my_char);
    my_char = my_char >> 1;
    println!("{:b}", my_char);
    my_char = my_char >> 1;
    println!("{:b}", my_char);
    my_char = my_char >> 1;
    println!("{:b}", my_char);
    my_char = my_char >> 1;
    println!("{:b}", my_char);
    my_char = my_char >> 1;
    println!("{:#08b}", 0b01);
    
}
