#![warn(clippy::excessive_precision)]
#![allow(
    dead_code,
    overflowing_literals,
    unused_variables,
    clippy::print_literal,
    clippy::useless_vec
)]

fn main() {
    // Consts
    const GOOD32: f32 = 0.123_456;
    const GOOD32_SM: f32 = 0.000_000_000_1;
    const GOOD32_DOT: f32 = 10_000_000_000.0;
    const GOOD32_EDGE: f32 = 1.000_000_8;
    const GOOD64: f64 = 0.123_456_789_012;
    const GOOD64_SM: f32 = 0.000_000_000_000_000_1;
    const GOOD64_DOT: f32 = 10_000_000_000_000_000.0;

    const BAD32_1: f32 = 0.123_456_789_f32;
    //~^ excessive_precision
    const BAD32_2: f32 = 0.123_456_789;
    //~^ excessive_precision
    const BAD32_3: f32 = 0.100_000_000_000_1;
    //~^ excessive_precision
    const BAD32_EDGE: f32 = 1.000_000_9;
    //~^ excessive_precision

    const BAD64_1: f64 = 0.123_456_789_012_345_67f64;
    const BAD64_2: f64 = 0.123_456_789_012_345_67;
    const BAD64_3: f64 = 0.100_000_000_000_000_000_1;
    //~^ excessive_precision

    // Literal as param
    println!("{:?}", 8.888_888_888_888_888_888_888);
    //~^ excessive_precision

    // // TODO add inferred type tests for f32
    // Locals
    let good32: f32 = 0.123_456_f32;
    let good32_2: f32 = 0.123_456;

    let good64: f64 = 0.123_456_789_012;
    let good64_suf: f64 = 0.123_456_789_012f64;
    let good64_inf = 0.123_456_789_012;

    let bad32: f32 = 1.123_456_789;
    //~^ excessive_precision
    let bad32_suf: f32 = 1.123_456_789_f32;
    //~^ excessive_precision
    let bad32_inf = 1.123_456_789_f32;
    //~^ excessive_precision

    let bad64: f64 = 0.123_456_789_012_345_67;
    let bad64_suf: f64 = 0.123_456_789_012_345_67f64;
    let bad64_inf = 0.123_456_789_012_345_67;

    // Vectors
    let good_vec32: Vec<f32> = vec![0.123_456];
    let good_vec64: Vec<f64> = vec![0.123_456_789];

    let bad_vec32: Vec<f32> = vec![0.123_456_789];
    //~^ excessive_precision
    let bad_vec64: Vec<f64> = vec![0.123_456_789_123_456_789];
    //~^ excessive_precision

    // Exponential float notation
    let good_e32: f32 = 1e-10;
    let bad_e32: f32 = 1.123_456_788_888e-10;
    //~^ excessive_precision

    let good_bige32: f32 = 1E-10;
    let bad_bige32: f32 = 1.123_456_788_888E-10;
    //~^ excessive_precision

    // Inferred type
    let good_inferred: f32 = 1f32 * 1_000_000_000.;

    // issue #2840
    let num = 0.000_000_000_01e-10f64;

    // issue #6341
    let exponential: f64 = 4.886506780521244E-03;

    // issue #7744
    let _ = 2.225_073_858_507_201_1e-308_f64;
    //~^ excessive_precision

    // issue #7745
    let _ = 1.000_000_000_000_001e-324_f64;
    //~^ excessive_precision

    // issue #9910
    const INF1: f32 = 1.0e+33f32;
    const INF2: f64 = 1.0e+3300f64;
    const NEG_INF1: f32 = -1.0e+33f32;
    const NEG_INF2: f64 = -1.0e+3300f64;
    const NEG_INF3: f32 = -3.40282357e+38_f32;

    // issue #12954
    const _: f64 = 3.0000000000000000e+00;
    //~^ excessive_precision
    const _: f64 = 3.0000000000000000;
}
