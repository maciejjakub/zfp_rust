use num_traits::Float;
use num_traits::cast::FromPrimitive;
use num_traits::cast::ToPrimitive;
use std::fmt::Debug;

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
    let base = 2.0f64;
    for i in 0..4 {
        let (mantissa, exponent, sign) = Float::integer_decode(float_arr[i]);
        println!("mantissa {:?}", mantissa);
        println!("exponent {:?}", exponent);
        println!("sign {:?}", sign);
        exp_array[i] = exponent as i64;
    }
    let max_exp = *exp_array.iter().max().unwrap() as i32;
    for j in 0..4 {
        let divisor = base.powi(max_exp);
        let divisor = I::from_f64(divisor).unwrap();
        let mantissa_fixed = float_arr[j] / divisor;
        mantissa_array[j] = I::to_i64(&mantissa_fixed.round()).unwrap();
    }
    (mantissa_array, max_exp)
}