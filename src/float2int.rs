use num_traits::Float;
use num_traits::cast::FromPrimitive;
use num_traits::cast::ToPrimitive;
use std::fmt::Debug;
use std::mem::size_of;

pub trait F3264:
    Debug + Float + FromPrimitive + ToPrimitive
{
}
impl<
        T: Debug
            + Float
            + FromPrimitive
            + ToPrimitive,
    > F3264 for T
{
}

pub fn test2(num: f32) {
    // let num = -6567.3616f32;
    println!("{:?}", num);

    let base = 2.0f32;
    
    // (8388608, -22, 1)
    let (mantissa, exponent, sign) = Float::integer_decode(num);
    println!("mantissa {:?}", mantissa);
    println!("exponent {:?}", exponent);
    println!("sign {:?}", sign);

    let sign_f = sign as f32;
    let mantissa_f = mantissa as f32;
    let exponent_f = num.powf(exponent as f32);

    let mantissa2 = mantissa as i64;
    let exponent2 = exponent as i32;
    let sign2 = sign as i64;

    // println!("{:?}", 2.powi(2));
    println!("original: {:?}", sign_f * mantissa_f * base.powi(exponent2));
}

pub fn float2int1<I: F3264>(float_arr: [I; 4]) -> ([i64; 4], i32) {
    let mut exp_array = [0; 4];
    let mut mantissa_array: [i64; 4] = [0; 4];
    let mut int_array: [i64; 4] = [0; 4];
    let BASE = 2.0f64;
    let EBITS = if size_of::<I>() == 4 { 8 } else { 11 };  // 8 bits exponent size if f32, 11 if f64
    let MANTISSA_SIZE = if size_of::<I>() == 4 { 24 } else { 53 };
    let EBIAS = ((1 << (EBITS - 1)) - 1);

    for i in 0..4 {
        println!("float number {:?}", float_arr[i]);
        let (mantissa, exponent, sign) = Float::integer_decode(float_arr[i]);
        println!("mantissa (binary) {:b}", mantissa);
        println!("mantissa (decimal) {:?}", mantissa);
        println!("exponent (binary) {:b}", exponent);
        println!("exponent (decimal) {:?}", exponent);
        println!("sign {:?}", sign);
        println!("sizeof {:?}", size_of::<I>());
        let exponent_normalized = exponent + MANTISSA_SIZE;
        exp_array[i] = exponent_normalized as i64;
    }
    let max_exp = *exp_array.iter().max().unwrap() as i32;
    let e: i32 = max_exp + EBIAS;
    let hlpr = (8 * size_of::<I>() - 2) as i32;
    let s = BASE.powi(hlpr - max_exp);
    let s = I::from_f64(s).unwrap();
    println!("s value: {:?}", s);
    for j in 0..4 {
        let int_element = s * float_arr[j];
        int_array[j] = I::to_i64(&int_element).unwrap();
        // let divisor = BASE.powi(max_exp);
        // let divisor = I::from_f64(divisor).unwrap();
        // let mantissa_fixed = float_arr[j] / divisor;
        // mantissa_array[j] = I::to_i64(&mantissa_fixed.round()).unwrap();
    }
    (int_array, max_exp)
}
