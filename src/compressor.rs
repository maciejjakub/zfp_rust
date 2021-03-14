
pub fn compress1() {
    let mut v: [i8; 4] = [67, 56, 43, 30];
    println!("Array before transformation: {:?}", v);
    super::encode::encode_cube1(&mut v);
    println!("Array after forward lifting transform: {:?}", v);
    v = super::fwd_order::fwd_order1(v);
    println!("Array after forward ordering: {:?}", v);
    let mut negabin_arr = [0; 4];
    for i in 0..4 {
        negabin_arr[i] = super::negbin_convert::Int2Uint::int2uint(v[i])
    }
    println!("Array after negabinary conversion: {:?}", negabin_arr);
    super::serialize::bit_transpose_vector(&negabin_arr);
}

pub fn compress2() {
    let mut v: [i8; 16] = [67, 56, 43, 21, 67, 56, 43, 21, 67, 56, 43, 21, 67, 56, 43, 21];
    println!("Array before transformation: {:?}", v);
    super::encode::encode_cube2(&mut v);
    println!("Array after forward lifting transform: {:?}", v);
    v = super::fwd_order::fwd_order2(v);
    println!("Array after forward ordering: {:?}", v);
    let mut negabin_arr = [0; 16];
    for i in 0..16 {
        negabin_arr[i] = super::negbin_convert::Int2Uint::int2uint(v[i])
    }
    println!("Array after negabinary conversion: {:?}", negabin_arr);
    super::serialize::bit_transpose_vector(&negabin_arr);
}

pub fn decompress1() {
    let mut v: [i8; 4] = [67, 56, 43, 30];
    println!("Array before transformation: {:?}", v);
    super::encode::encode_cube1(&mut v);
    println!("Array after forward lifting transform: {:?}", v);
    v = super::fwd_order::fwd_order1(v);
    println!("Array after forward ordering: {:?}", v);
    let mut negabin_arr = [0; 4];
    for i in 0..4 {
        negabin_arr[i] = super::negbin_convert::Int2Uint::int2uint(v[i])
    }
    println!("Array after negabinary conversion: {:?}", negabin_arr);
    super::serialize::bit_transpose_vector(&negabin_arr);
}

// pub fn load_netcdf() -> () {
//     let file = netcdf::open("/home/milismac/Dokumenty/mgr/data")?;
// }