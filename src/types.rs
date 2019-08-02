// Copyright © 2018–2019 Trevor Spiteri

// This library is free software: you can redistribute it and/or
// modify it under the terms of either
//
//   * the Apache License, Version 2.0 or
//   * the MIT License
//
// at your option.
//
// You should have recieved copies of the Apache License and the MIT
// License along with the library. If not, see
// <https://www.apache.org/licenses/LICENSE-2.0> and
// <https://opensource.org/licenses/MIT>.

/*!
This module provides type aliases for all supported fixed-point
numbers.
*/

use crate::frac;
use crate::{
    FixedI128, FixedI16, FixedI32, FixedI64, FixedI8, FixedU128, FixedU16, FixedU32, FixedU64,
    FixedU8,
};

/*
```rust
fn num(n: i32, noun: &str) -> String {
    let mut ret = match n {
        0 => "no".to_string(),
        1 => "one".to_string(),
        2 => "two".to_string(),
        3 => "three".to_string(),
        4 => "four".to_string(),
        5 => "five".to_string(),
        6 => "six".to_string(),
        7 => "seven".to_string(),
        8 => "eight".to_string(),
        9 => "nine".to_string(),
        _ => n.to_string(),
    };
    ret.push_str(" ");
    ret.push_str(noun);
    if n != 1 {
        ret.push_str("s");
    }
    ret
}

fn main() {
    for &sign in &["I", "U"] {
        for &prim_bits in &[8, 16, 32, 64, 128] {
            for frac_bits in 0..=prim_bits {
                let int_bits = prim_bits - frac_bits;
                let int_desc = num(int_bits, "integer bit");
                let frac_desc = num(frac_bits, "fractional bit");
                println!(
                    "/// [`Fixed{0}{1}`](../struct.Fixed{0}{1}.html) with {2} and {3}.",
                    sign, prim_bits, int_desc, frac_desc,
                );
                println!(
                    "pub type {0}{2}F{3} = Fixed{0}{1}<frac::U{3}>;",
                    sign, prim_bits, int_bits, frac_bits,
                );
            }
        }
    }
}
```
*/

/// [`FixedI8`](../struct.FixedI8.html) with eight integer bits and no fractional bits.
pub type I8F0 = FixedI8<frac::U0>;
/// [`FixedI8`](../struct.FixedI8.html) with seven integer bits and one fractional bit.
pub type I7F1 = FixedI8<frac::U1>;
/// [`FixedI8`](../struct.FixedI8.html) with six integer bits and two fractional bits.
pub type I6F2 = FixedI8<frac::U2>;
/// [`FixedI8`](../struct.FixedI8.html) with five integer bits and three fractional bits.
pub type I5F3 = FixedI8<frac::U3>;
/// [`FixedI8`](../struct.FixedI8.html) with four integer bits and four fractional bits.
pub type I4F4 = FixedI8<frac::U4>;
/// [`FixedI8`](../struct.FixedI8.html) with three integer bits and five fractional bits.
pub type I3F5 = FixedI8<frac::U5>;
/// [`FixedI8`](../struct.FixedI8.html) with two integer bits and six fractional bits.
pub type I2F6 = FixedI8<frac::U6>;
/// [`FixedI8`](../struct.FixedI8.html) with one integer bit and seven fractional bits.
pub type I1F7 = FixedI8<frac::U7>;
/// [`FixedI8`](../struct.FixedI8.html) with no integer bits and eight fractional bits.
pub type I0F8 = FixedI8<frac::U8>;
/// [`FixedI16`](../struct.FixedI16.html) with 16 integer bits and no fractional bits.
pub type I16F0 = FixedI16<frac::U0>;
/// [`FixedI16`](../struct.FixedI16.html) with 15 integer bits and one fractional bit.
pub type I15F1 = FixedI16<frac::U1>;
/// [`FixedI16`](../struct.FixedI16.html) with 14 integer bits and two fractional bits.
pub type I14F2 = FixedI16<frac::U2>;
/// [`FixedI16`](../struct.FixedI16.html) with 13 integer bits and three fractional bits.
pub type I13F3 = FixedI16<frac::U3>;
/// [`FixedI16`](../struct.FixedI16.html) with 12 integer bits and four fractional bits.
pub type I12F4 = FixedI16<frac::U4>;
/// [`FixedI16`](../struct.FixedI16.html) with 11 integer bits and five fractional bits.
pub type I11F5 = FixedI16<frac::U5>;
/// [`FixedI16`](../struct.FixedI16.html) with 10 integer bits and six fractional bits.
pub type I10F6 = FixedI16<frac::U6>;
/// [`FixedI16`](../struct.FixedI16.html) with nine integer bits and seven fractional bits.
pub type I9F7 = FixedI16<frac::U7>;
/// [`FixedI16`](../struct.FixedI16.html) with eight integer bits and eight fractional bits.
pub type I8F8 = FixedI16<frac::U8>;
/// [`FixedI16`](../struct.FixedI16.html) with seven integer bits and nine fractional bits.
pub type I7F9 = FixedI16<frac::U9>;
/// [`FixedI16`](../struct.FixedI16.html) with six integer bits and 10 fractional bits.
pub type I6F10 = FixedI16<frac::U10>;
/// [`FixedI16`](../struct.FixedI16.html) with five integer bits and 11 fractional bits.
pub type I5F11 = FixedI16<frac::U11>;
/// [`FixedI16`](../struct.FixedI16.html) with four integer bits and 12 fractional bits.
pub type I4F12 = FixedI16<frac::U12>;
/// [`FixedI16`](../struct.FixedI16.html) with three integer bits and 13 fractional bits.
pub type I3F13 = FixedI16<frac::U13>;
/// [`FixedI16`](../struct.FixedI16.html) with two integer bits and 14 fractional bits.
pub type I2F14 = FixedI16<frac::U14>;
/// [`FixedI16`](../struct.FixedI16.html) with one integer bit and 15 fractional bits.
pub type I1F15 = FixedI16<frac::U15>;
/// [`FixedI16`](../struct.FixedI16.html) with no integer bits and 16 fractional bits.
pub type I0F16 = FixedI16<frac::U16>;
/// [`FixedI32`](../struct.FixedI32.html) with 32 integer bits and no fractional bits.
pub type I32F0 = FixedI32<frac::U0>;
/// [`FixedI32`](../struct.FixedI32.html) with 31 integer bits and one fractional bit.
pub type I31F1 = FixedI32<frac::U1>;
/// [`FixedI32`](../struct.FixedI32.html) with 30 integer bits and two fractional bits.
pub type I30F2 = FixedI32<frac::U2>;
/// [`FixedI32`](../struct.FixedI32.html) with 29 integer bits and three fractional bits.
pub type I29F3 = FixedI32<frac::U3>;
/// [`FixedI32`](../struct.FixedI32.html) with 28 integer bits and four fractional bits.
pub type I28F4 = FixedI32<frac::U4>;
/// [`FixedI32`](../struct.FixedI32.html) with 27 integer bits and five fractional bits.
pub type I27F5 = FixedI32<frac::U5>;
/// [`FixedI32`](../struct.FixedI32.html) with 26 integer bits and six fractional bits.
pub type I26F6 = FixedI32<frac::U6>;
/// [`FixedI32`](../struct.FixedI32.html) with 25 integer bits and seven fractional bits.
pub type I25F7 = FixedI32<frac::U7>;
/// [`FixedI32`](../struct.FixedI32.html) with 24 integer bits and eight fractional bits.
pub type I24F8 = FixedI32<frac::U8>;
/// [`FixedI32`](../struct.FixedI32.html) with 23 integer bits and nine fractional bits.
pub type I23F9 = FixedI32<frac::U9>;
/// [`FixedI32`](../struct.FixedI32.html) with 22 integer bits and 10 fractional bits.
pub type I22F10 = FixedI32<frac::U10>;
/// [`FixedI32`](../struct.FixedI32.html) with 21 integer bits and 11 fractional bits.
pub type I21F11 = FixedI32<frac::U11>;
/// [`FixedI32`](../struct.FixedI32.html) with 20 integer bits and 12 fractional bits.
pub type I20F12 = FixedI32<frac::U12>;
/// [`FixedI32`](../struct.FixedI32.html) with 19 integer bits and 13 fractional bits.
pub type I19F13 = FixedI32<frac::U13>;
/// [`FixedI32`](../struct.FixedI32.html) with 18 integer bits and 14 fractional bits.
pub type I18F14 = FixedI32<frac::U14>;
/// [`FixedI32`](../struct.FixedI32.html) with 17 integer bits and 15 fractional bits.
pub type I17F15 = FixedI32<frac::U15>;
/// [`FixedI32`](../struct.FixedI32.html) with 16 integer bits and 16 fractional bits.
pub type I16F16 = FixedI32<frac::U16>;
/// [`FixedI32`](../struct.FixedI32.html) with 15 integer bits and 17 fractional bits.
pub type I15F17 = FixedI32<frac::U17>;
/// [`FixedI32`](../struct.FixedI32.html) with 14 integer bits and 18 fractional bits.
pub type I14F18 = FixedI32<frac::U18>;
/// [`FixedI32`](../struct.FixedI32.html) with 13 integer bits and 19 fractional bits.
pub type I13F19 = FixedI32<frac::U19>;
/// [`FixedI32`](../struct.FixedI32.html) with 12 integer bits and 20 fractional bits.
pub type I12F20 = FixedI32<frac::U20>;
/// [`FixedI32`](../struct.FixedI32.html) with 11 integer bits and 21 fractional bits.
pub type I11F21 = FixedI32<frac::U21>;
/// [`FixedI32`](../struct.FixedI32.html) with 10 integer bits and 22 fractional bits.
pub type I10F22 = FixedI32<frac::U22>;
/// [`FixedI32`](../struct.FixedI32.html) with nine integer bits and 23 fractional bits.
pub type I9F23 = FixedI32<frac::U23>;
/// [`FixedI32`](../struct.FixedI32.html) with eight integer bits and 24 fractional bits.
pub type I8F24 = FixedI32<frac::U24>;
/// [`FixedI32`](../struct.FixedI32.html) with seven integer bits and 25 fractional bits.
pub type I7F25 = FixedI32<frac::U25>;
/// [`FixedI32`](../struct.FixedI32.html) with six integer bits and 26 fractional bits.
pub type I6F26 = FixedI32<frac::U26>;
/// [`FixedI32`](../struct.FixedI32.html) with five integer bits and 27 fractional bits.
pub type I5F27 = FixedI32<frac::U27>;
/// [`FixedI32`](../struct.FixedI32.html) with four integer bits and 28 fractional bits.
pub type I4F28 = FixedI32<frac::U28>;
/// [`FixedI32`](../struct.FixedI32.html) with three integer bits and 29 fractional bits.
pub type I3F29 = FixedI32<frac::U29>;
/// [`FixedI32`](../struct.FixedI32.html) with two integer bits and 30 fractional bits.
pub type I2F30 = FixedI32<frac::U30>;
/// [`FixedI32`](../struct.FixedI32.html) with one integer bit and 31 fractional bits.
pub type I1F31 = FixedI32<frac::U31>;
/// [`FixedI32`](../struct.FixedI32.html) with no integer bits and 32 fractional bits.
pub type I0F32 = FixedI32<frac::U32>;
/// [`FixedI64`](../struct.FixedI64.html) with 64 integer bits and no fractional bits.
pub type I64F0 = FixedI64<frac::U0>;
/// [`FixedI64`](../struct.FixedI64.html) with 63 integer bits and one fractional bit.
pub type I63F1 = FixedI64<frac::U1>;
/// [`FixedI64`](../struct.FixedI64.html) with 62 integer bits and two fractional bits.
pub type I62F2 = FixedI64<frac::U2>;
/// [`FixedI64`](../struct.FixedI64.html) with 61 integer bits and three fractional bits.
pub type I61F3 = FixedI64<frac::U3>;
/// [`FixedI64`](../struct.FixedI64.html) with 60 integer bits and four fractional bits.
pub type I60F4 = FixedI64<frac::U4>;
/// [`FixedI64`](../struct.FixedI64.html) with 59 integer bits and five fractional bits.
pub type I59F5 = FixedI64<frac::U5>;
/// [`FixedI64`](../struct.FixedI64.html) with 58 integer bits and six fractional bits.
pub type I58F6 = FixedI64<frac::U6>;
/// [`FixedI64`](../struct.FixedI64.html) with 57 integer bits and seven fractional bits.
pub type I57F7 = FixedI64<frac::U7>;
/// [`FixedI64`](../struct.FixedI64.html) with 56 integer bits and eight fractional bits.
pub type I56F8 = FixedI64<frac::U8>;
/// [`FixedI64`](../struct.FixedI64.html) with 55 integer bits and nine fractional bits.
pub type I55F9 = FixedI64<frac::U9>;
/// [`FixedI64`](../struct.FixedI64.html) with 54 integer bits and 10 fractional bits.
pub type I54F10 = FixedI64<frac::U10>;
/// [`FixedI64`](../struct.FixedI64.html) with 53 integer bits and 11 fractional bits.
pub type I53F11 = FixedI64<frac::U11>;
/// [`FixedI64`](../struct.FixedI64.html) with 52 integer bits and 12 fractional bits.
pub type I52F12 = FixedI64<frac::U12>;
/// [`FixedI64`](../struct.FixedI64.html) with 51 integer bits and 13 fractional bits.
pub type I51F13 = FixedI64<frac::U13>;
/// [`FixedI64`](../struct.FixedI64.html) with 50 integer bits and 14 fractional bits.
pub type I50F14 = FixedI64<frac::U14>;
/// [`FixedI64`](../struct.FixedI64.html) with 49 integer bits and 15 fractional bits.
pub type I49F15 = FixedI64<frac::U15>;
/// [`FixedI64`](../struct.FixedI64.html) with 48 integer bits and 16 fractional bits.
pub type I48F16 = FixedI64<frac::U16>;
/// [`FixedI64`](../struct.FixedI64.html) with 47 integer bits and 17 fractional bits.
pub type I47F17 = FixedI64<frac::U17>;
/// [`FixedI64`](../struct.FixedI64.html) with 46 integer bits and 18 fractional bits.
pub type I46F18 = FixedI64<frac::U18>;
/// [`FixedI64`](../struct.FixedI64.html) with 45 integer bits and 19 fractional bits.
pub type I45F19 = FixedI64<frac::U19>;
/// [`FixedI64`](../struct.FixedI64.html) with 44 integer bits and 20 fractional bits.
pub type I44F20 = FixedI64<frac::U20>;
/// [`FixedI64`](../struct.FixedI64.html) with 43 integer bits and 21 fractional bits.
pub type I43F21 = FixedI64<frac::U21>;
/// [`FixedI64`](../struct.FixedI64.html) with 42 integer bits and 22 fractional bits.
pub type I42F22 = FixedI64<frac::U22>;
/// [`FixedI64`](../struct.FixedI64.html) with 41 integer bits and 23 fractional bits.
pub type I41F23 = FixedI64<frac::U23>;
/// [`FixedI64`](../struct.FixedI64.html) with 40 integer bits and 24 fractional bits.
pub type I40F24 = FixedI64<frac::U24>;
/// [`FixedI64`](../struct.FixedI64.html) with 39 integer bits and 25 fractional bits.
pub type I39F25 = FixedI64<frac::U25>;
/// [`FixedI64`](../struct.FixedI64.html) with 38 integer bits and 26 fractional bits.
pub type I38F26 = FixedI64<frac::U26>;
/// [`FixedI64`](../struct.FixedI64.html) with 37 integer bits and 27 fractional bits.
pub type I37F27 = FixedI64<frac::U27>;
/// [`FixedI64`](../struct.FixedI64.html) with 36 integer bits and 28 fractional bits.
pub type I36F28 = FixedI64<frac::U28>;
/// [`FixedI64`](../struct.FixedI64.html) with 35 integer bits and 29 fractional bits.
pub type I35F29 = FixedI64<frac::U29>;
/// [`FixedI64`](../struct.FixedI64.html) with 34 integer bits and 30 fractional bits.
pub type I34F30 = FixedI64<frac::U30>;
/// [`FixedI64`](../struct.FixedI64.html) with 33 integer bits and 31 fractional bits.
pub type I33F31 = FixedI64<frac::U31>;
/// [`FixedI64`](../struct.FixedI64.html) with 32 integer bits and 32 fractional bits.
pub type I32F32 = FixedI64<frac::U32>;
/// [`FixedI64`](../struct.FixedI64.html) with 31 integer bits and 33 fractional bits.
pub type I31F33 = FixedI64<frac::U33>;
/// [`FixedI64`](../struct.FixedI64.html) with 30 integer bits and 34 fractional bits.
pub type I30F34 = FixedI64<frac::U34>;
/// [`FixedI64`](../struct.FixedI64.html) with 29 integer bits and 35 fractional bits.
pub type I29F35 = FixedI64<frac::U35>;
/// [`FixedI64`](../struct.FixedI64.html) with 28 integer bits and 36 fractional bits.
pub type I28F36 = FixedI64<frac::U36>;
/// [`FixedI64`](../struct.FixedI64.html) with 27 integer bits and 37 fractional bits.
pub type I27F37 = FixedI64<frac::U37>;
/// [`FixedI64`](../struct.FixedI64.html) with 26 integer bits and 38 fractional bits.
pub type I26F38 = FixedI64<frac::U38>;
/// [`FixedI64`](../struct.FixedI64.html) with 25 integer bits and 39 fractional bits.
pub type I25F39 = FixedI64<frac::U39>;
/// [`FixedI64`](../struct.FixedI64.html) with 24 integer bits and 40 fractional bits.
pub type I24F40 = FixedI64<frac::U40>;
/// [`FixedI64`](../struct.FixedI64.html) with 23 integer bits and 41 fractional bits.
pub type I23F41 = FixedI64<frac::U41>;
/// [`FixedI64`](../struct.FixedI64.html) with 22 integer bits and 42 fractional bits.
pub type I22F42 = FixedI64<frac::U42>;
/// [`FixedI64`](../struct.FixedI64.html) with 21 integer bits and 43 fractional bits.
pub type I21F43 = FixedI64<frac::U43>;
/// [`FixedI64`](../struct.FixedI64.html) with 20 integer bits and 44 fractional bits.
pub type I20F44 = FixedI64<frac::U44>;
/// [`FixedI64`](../struct.FixedI64.html) with 19 integer bits and 45 fractional bits.
pub type I19F45 = FixedI64<frac::U45>;
/// [`FixedI64`](../struct.FixedI64.html) with 18 integer bits and 46 fractional bits.
pub type I18F46 = FixedI64<frac::U46>;
/// [`FixedI64`](../struct.FixedI64.html) with 17 integer bits and 47 fractional bits.
pub type I17F47 = FixedI64<frac::U47>;
/// [`FixedI64`](../struct.FixedI64.html) with 16 integer bits and 48 fractional bits.
pub type I16F48 = FixedI64<frac::U48>;
/// [`FixedI64`](../struct.FixedI64.html) with 15 integer bits and 49 fractional bits.
pub type I15F49 = FixedI64<frac::U49>;
/// [`FixedI64`](../struct.FixedI64.html) with 14 integer bits and 50 fractional bits.
pub type I14F50 = FixedI64<frac::U50>;
/// [`FixedI64`](../struct.FixedI64.html) with 13 integer bits and 51 fractional bits.
pub type I13F51 = FixedI64<frac::U51>;
/// [`FixedI64`](../struct.FixedI64.html) with 12 integer bits and 52 fractional bits.
pub type I12F52 = FixedI64<frac::U52>;
/// [`FixedI64`](../struct.FixedI64.html) with 11 integer bits and 53 fractional bits.
pub type I11F53 = FixedI64<frac::U53>;
/// [`FixedI64`](../struct.FixedI64.html) with 10 integer bits and 54 fractional bits.
pub type I10F54 = FixedI64<frac::U54>;
/// [`FixedI64`](../struct.FixedI64.html) with nine integer bits and 55 fractional bits.
pub type I9F55 = FixedI64<frac::U55>;
/// [`FixedI64`](../struct.FixedI64.html) with eight integer bits and 56 fractional bits.
pub type I8F56 = FixedI64<frac::U56>;
/// [`FixedI64`](../struct.FixedI64.html) with seven integer bits and 57 fractional bits.
pub type I7F57 = FixedI64<frac::U57>;
/// [`FixedI64`](../struct.FixedI64.html) with six integer bits and 58 fractional bits.
pub type I6F58 = FixedI64<frac::U58>;
/// [`FixedI64`](../struct.FixedI64.html) with five integer bits and 59 fractional bits.
pub type I5F59 = FixedI64<frac::U59>;
/// [`FixedI64`](../struct.FixedI64.html) with four integer bits and 60 fractional bits.
pub type I4F60 = FixedI64<frac::U60>;
/// [`FixedI64`](../struct.FixedI64.html) with three integer bits and 61 fractional bits.
pub type I3F61 = FixedI64<frac::U61>;
/// [`FixedI64`](../struct.FixedI64.html) with two integer bits and 62 fractional bits.
pub type I2F62 = FixedI64<frac::U62>;
/// [`FixedI64`](../struct.FixedI64.html) with one integer bit and 63 fractional bits.
pub type I1F63 = FixedI64<frac::U63>;
/// [`FixedI64`](../struct.FixedI64.html) with no integer bits and 64 fractional bits.
pub type I0F64 = FixedI64<frac::U64>;
/// [`FixedI128`](../struct.FixedI128.html) with 128 integer bits and no fractional bits.
pub type I128F0 = FixedI128<frac::U0>;
/// [`FixedI128`](../struct.FixedI128.html) with 127 integer bits and one fractional bit.
pub type I127F1 = FixedI128<frac::U1>;
/// [`FixedI128`](../struct.FixedI128.html) with 126 integer bits and two fractional bits.
pub type I126F2 = FixedI128<frac::U2>;
/// [`FixedI128`](../struct.FixedI128.html) with 125 integer bits and three fractional bits.
pub type I125F3 = FixedI128<frac::U3>;
/// [`FixedI128`](../struct.FixedI128.html) with 124 integer bits and four fractional bits.
pub type I124F4 = FixedI128<frac::U4>;
/// [`FixedI128`](../struct.FixedI128.html) with 123 integer bits and five fractional bits.
pub type I123F5 = FixedI128<frac::U5>;
/// [`FixedI128`](../struct.FixedI128.html) with 122 integer bits and six fractional bits.
pub type I122F6 = FixedI128<frac::U6>;
/// [`FixedI128`](../struct.FixedI128.html) with 121 integer bits and seven fractional bits.
pub type I121F7 = FixedI128<frac::U7>;
/// [`FixedI128`](../struct.FixedI128.html) with 120 integer bits and eight fractional bits.
pub type I120F8 = FixedI128<frac::U8>;
/// [`FixedI128`](../struct.FixedI128.html) with 119 integer bits and nine fractional bits.
pub type I119F9 = FixedI128<frac::U9>;
/// [`FixedI128`](../struct.FixedI128.html) with 118 integer bits and 10 fractional bits.
pub type I118F10 = FixedI128<frac::U10>;
/// [`FixedI128`](../struct.FixedI128.html) with 117 integer bits and 11 fractional bits.
pub type I117F11 = FixedI128<frac::U11>;
/// [`FixedI128`](../struct.FixedI128.html) with 116 integer bits and 12 fractional bits.
pub type I116F12 = FixedI128<frac::U12>;
/// [`FixedI128`](../struct.FixedI128.html) with 115 integer bits and 13 fractional bits.
pub type I115F13 = FixedI128<frac::U13>;
/// [`FixedI128`](../struct.FixedI128.html) with 114 integer bits and 14 fractional bits.
pub type I114F14 = FixedI128<frac::U14>;
/// [`FixedI128`](../struct.FixedI128.html) with 113 integer bits and 15 fractional bits.
pub type I113F15 = FixedI128<frac::U15>;
/// [`FixedI128`](../struct.FixedI128.html) with 112 integer bits and 16 fractional bits.
pub type I112F16 = FixedI128<frac::U16>;
/// [`FixedI128`](../struct.FixedI128.html) with 111 integer bits and 17 fractional bits.
pub type I111F17 = FixedI128<frac::U17>;
/// [`FixedI128`](../struct.FixedI128.html) with 110 integer bits and 18 fractional bits.
pub type I110F18 = FixedI128<frac::U18>;
/// [`FixedI128`](../struct.FixedI128.html) with 109 integer bits and 19 fractional bits.
pub type I109F19 = FixedI128<frac::U19>;
/// [`FixedI128`](../struct.FixedI128.html) with 108 integer bits and 20 fractional bits.
pub type I108F20 = FixedI128<frac::U20>;
/// [`FixedI128`](../struct.FixedI128.html) with 107 integer bits and 21 fractional bits.
pub type I107F21 = FixedI128<frac::U21>;
/// [`FixedI128`](../struct.FixedI128.html) with 106 integer bits and 22 fractional bits.
pub type I106F22 = FixedI128<frac::U22>;
/// [`FixedI128`](../struct.FixedI128.html) with 105 integer bits and 23 fractional bits.
pub type I105F23 = FixedI128<frac::U23>;
/// [`FixedI128`](../struct.FixedI128.html) with 104 integer bits and 24 fractional bits.
pub type I104F24 = FixedI128<frac::U24>;
/// [`FixedI128`](../struct.FixedI128.html) with 103 integer bits and 25 fractional bits.
pub type I103F25 = FixedI128<frac::U25>;
/// [`FixedI128`](../struct.FixedI128.html) with 102 integer bits and 26 fractional bits.
pub type I102F26 = FixedI128<frac::U26>;
/// [`FixedI128`](../struct.FixedI128.html) with 101 integer bits and 27 fractional bits.
pub type I101F27 = FixedI128<frac::U27>;
/// [`FixedI128`](../struct.FixedI128.html) with 100 integer bits and 28 fractional bits.
pub type I100F28 = FixedI128<frac::U28>;
/// [`FixedI128`](../struct.FixedI128.html) with 99 integer bits and 29 fractional bits.
pub type I99F29 = FixedI128<frac::U29>;
/// [`FixedI128`](../struct.FixedI128.html) with 98 integer bits and 30 fractional bits.
pub type I98F30 = FixedI128<frac::U30>;
/// [`FixedI128`](../struct.FixedI128.html) with 97 integer bits and 31 fractional bits.
pub type I97F31 = FixedI128<frac::U31>;
/// [`FixedI128`](../struct.FixedI128.html) with 96 integer bits and 32 fractional bits.
pub type I96F32 = FixedI128<frac::U32>;
/// [`FixedI128`](../struct.FixedI128.html) with 95 integer bits and 33 fractional bits.
pub type I95F33 = FixedI128<frac::U33>;
/// [`FixedI128`](../struct.FixedI128.html) with 94 integer bits and 34 fractional bits.
pub type I94F34 = FixedI128<frac::U34>;
/// [`FixedI128`](../struct.FixedI128.html) with 93 integer bits and 35 fractional bits.
pub type I93F35 = FixedI128<frac::U35>;
/// [`FixedI128`](../struct.FixedI128.html) with 92 integer bits and 36 fractional bits.
pub type I92F36 = FixedI128<frac::U36>;
/// [`FixedI128`](../struct.FixedI128.html) with 91 integer bits and 37 fractional bits.
pub type I91F37 = FixedI128<frac::U37>;
/// [`FixedI128`](../struct.FixedI128.html) with 90 integer bits and 38 fractional bits.
pub type I90F38 = FixedI128<frac::U38>;
/// [`FixedI128`](../struct.FixedI128.html) with 89 integer bits and 39 fractional bits.
pub type I89F39 = FixedI128<frac::U39>;
/// [`FixedI128`](../struct.FixedI128.html) with 88 integer bits and 40 fractional bits.
pub type I88F40 = FixedI128<frac::U40>;
/// [`FixedI128`](../struct.FixedI128.html) with 87 integer bits and 41 fractional bits.
pub type I87F41 = FixedI128<frac::U41>;
/// [`FixedI128`](../struct.FixedI128.html) with 86 integer bits and 42 fractional bits.
pub type I86F42 = FixedI128<frac::U42>;
/// [`FixedI128`](../struct.FixedI128.html) with 85 integer bits and 43 fractional bits.
pub type I85F43 = FixedI128<frac::U43>;
/// [`FixedI128`](../struct.FixedI128.html) with 84 integer bits and 44 fractional bits.
pub type I84F44 = FixedI128<frac::U44>;
/// [`FixedI128`](../struct.FixedI128.html) with 83 integer bits and 45 fractional bits.
pub type I83F45 = FixedI128<frac::U45>;
/// [`FixedI128`](../struct.FixedI128.html) with 82 integer bits and 46 fractional bits.
pub type I82F46 = FixedI128<frac::U46>;
/// [`FixedI128`](../struct.FixedI128.html) with 81 integer bits and 47 fractional bits.
pub type I81F47 = FixedI128<frac::U47>;
/// [`FixedI128`](../struct.FixedI128.html) with 80 integer bits and 48 fractional bits.
pub type I80F48 = FixedI128<frac::U48>;
/// [`FixedI128`](../struct.FixedI128.html) with 79 integer bits and 49 fractional bits.
pub type I79F49 = FixedI128<frac::U49>;
/// [`FixedI128`](../struct.FixedI128.html) with 78 integer bits and 50 fractional bits.
pub type I78F50 = FixedI128<frac::U50>;
/// [`FixedI128`](../struct.FixedI128.html) with 77 integer bits and 51 fractional bits.
pub type I77F51 = FixedI128<frac::U51>;
/// [`FixedI128`](../struct.FixedI128.html) with 76 integer bits and 52 fractional bits.
pub type I76F52 = FixedI128<frac::U52>;
/// [`FixedI128`](../struct.FixedI128.html) with 75 integer bits and 53 fractional bits.
pub type I75F53 = FixedI128<frac::U53>;
/// [`FixedI128`](../struct.FixedI128.html) with 74 integer bits and 54 fractional bits.
pub type I74F54 = FixedI128<frac::U54>;
/// [`FixedI128`](../struct.FixedI128.html) with 73 integer bits and 55 fractional bits.
pub type I73F55 = FixedI128<frac::U55>;
/// [`FixedI128`](../struct.FixedI128.html) with 72 integer bits and 56 fractional bits.
pub type I72F56 = FixedI128<frac::U56>;
/// [`FixedI128`](../struct.FixedI128.html) with 71 integer bits and 57 fractional bits.
pub type I71F57 = FixedI128<frac::U57>;
/// [`FixedI128`](../struct.FixedI128.html) with 70 integer bits and 58 fractional bits.
pub type I70F58 = FixedI128<frac::U58>;
/// [`FixedI128`](../struct.FixedI128.html) with 69 integer bits and 59 fractional bits.
pub type I69F59 = FixedI128<frac::U59>;
/// [`FixedI128`](../struct.FixedI128.html) with 68 integer bits and 60 fractional bits.
pub type I68F60 = FixedI128<frac::U60>;
/// [`FixedI128`](../struct.FixedI128.html) with 67 integer bits and 61 fractional bits.
pub type I67F61 = FixedI128<frac::U61>;
/// [`FixedI128`](../struct.FixedI128.html) with 66 integer bits and 62 fractional bits.
pub type I66F62 = FixedI128<frac::U62>;
/// [`FixedI128`](../struct.FixedI128.html) with 65 integer bits and 63 fractional bits.
pub type I65F63 = FixedI128<frac::U63>;
/// [`FixedI128`](../struct.FixedI128.html) with 64 integer bits and 64 fractional bits.
pub type I64F64 = FixedI128<frac::U64>;
/// [`FixedI128`](../struct.FixedI128.html) with 63 integer bits and 65 fractional bits.
pub type I63F65 = FixedI128<frac::U65>;
/// [`FixedI128`](../struct.FixedI128.html) with 62 integer bits and 66 fractional bits.
pub type I62F66 = FixedI128<frac::U66>;
/// [`FixedI128`](../struct.FixedI128.html) with 61 integer bits and 67 fractional bits.
pub type I61F67 = FixedI128<frac::U67>;
/// [`FixedI128`](../struct.FixedI128.html) with 60 integer bits and 68 fractional bits.
pub type I60F68 = FixedI128<frac::U68>;
/// [`FixedI128`](../struct.FixedI128.html) with 59 integer bits and 69 fractional bits.
pub type I59F69 = FixedI128<frac::U69>;
/// [`FixedI128`](../struct.FixedI128.html) with 58 integer bits and 70 fractional bits.
pub type I58F70 = FixedI128<frac::U70>;
/// [`FixedI128`](../struct.FixedI128.html) with 57 integer bits and 71 fractional bits.
pub type I57F71 = FixedI128<frac::U71>;
/// [`FixedI128`](../struct.FixedI128.html) with 56 integer bits and 72 fractional bits.
pub type I56F72 = FixedI128<frac::U72>;
/// [`FixedI128`](../struct.FixedI128.html) with 55 integer bits and 73 fractional bits.
pub type I55F73 = FixedI128<frac::U73>;
/// [`FixedI128`](../struct.FixedI128.html) with 54 integer bits and 74 fractional bits.
pub type I54F74 = FixedI128<frac::U74>;
/// [`FixedI128`](../struct.FixedI128.html) with 53 integer bits and 75 fractional bits.
pub type I53F75 = FixedI128<frac::U75>;
/// [`FixedI128`](../struct.FixedI128.html) with 52 integer bits and 76 fractional bits.
pub type I52F76 = FixedI128<frac::U76>;
/// [`FixedI128`](../struct.FixedI128.html) with 51 integer bits and 77 fractional bits.
pub type I51F77 = FixedI128<frac::U77>;
/// [`FixedI128`](../struct.FixedI128.html) with 50 integer bits and 78 fractional bits.
pub type I50F78 = FixedI128<frac::U78>;
/// [`FixedI128`](../struct.FixedI128.html) with 49 integer bits and 79 fractional bits.
pub type I49F79 = FixedI128<frac::U79>;
/// [`FixedI128`](../struct.FixedI128.html) with 48 integer bits and 80 fractional bits.
pub type I48F80 = FixedI128<frac::U80>;
/// [`FixedI128`](../struct.FixedI128.html) with 47 integer bits and 81 fractional bits.
pub type I47F81 = FixedI128<frac::U81>;
/// [`FixedI128`](../struct.FixedI128.html) with 46 integer bits and 82 fractional bits.
pub type I46F82 = FixedI128<frac::U82>;
/// [`FixedI128`](../struct.FixedI128.html) with 45 integer bits and 83 fractional bits.
pub type I45F83 = FixedI128<frac::U83>;
/// [`FixedI128`](../struct.FixedI128.html) with 44 integer bits and 84 fractional bits.
pub type I44F84 = FixedI128<frac::U84>;
/// [`FixedI128`](../struct.FixedI128.html) with 43 integer bits and 85 fractional bits.
pub type I43F85 = FixedI128<frac::U85>;
/// [`FixedI128`](../struct.FixedI128.html) with 42 integer bits and 86 fractional bits.
pub type I42F86 = FixedI128<frac::U86>;
/// [`FixedI128`](../struct.FixedI128.html) with 41 integer bits and 87 fractional bits.
pub type I41F87 = FixedI128<frac::U87>;
/// [`FixedI128`](../struct.FixedI128.html) with 40 integer bits and 88 fractional bits.
pub type I40F88 = FixedI128<frac::U88>;
/// [`FixedI128`](../struct.FixedI128.html) with 39 integer bits and 89 fractional bits.
pub type I39F89 = FixedI128<frac::U89>;
/// [`FixedI128`](../struct.FixedI128.html) with 38 integer bits and 90 fractional bits.
pub type I38F90 = FixedI128<frac::U90>;
/// [`FixedI128`](../struct.FixedI128.html) with 37 integer bits and 91 fractional bits.
pub type I37F91 = FixedI128<frac::U91>;
/// [`FixedI128`](../struct.FixedI128.html) with 36 integer bits and 92 fractional bits.
pub type I36F92 = FixedI128<frac::U92>;
/// [`FixedI128`](../struct.FixedI128.html) with 35 integer bits and 93 fractional bits.
pub type I35F93 = FixedI128<frac::U93>;
/// [`FixedI128`](../struct.FixedI128.html) with 34 integer bits and 94 fractional bits.
pub type I34F94 = FixedI128<frac::U94>;
/// [`FixedI128`](../struct.FixedI128.html) with 33 integer bits and 95 fractional bits.
pub type I33F95 = FixedI128<frac::U95>;
/// [`FixedI128`](../struct.FixedI128.html) with 32 integer bits and 96 fractional bits.
pub type I32F96 = FixedI128<frac::U96>;
/// [`FixedI128`](../struct.FixedI128.html) with 31 integer bits and 97 fractional bits.
pub type I31F97 = FixedI128<frac::U97>;
/// [`FixedI128`](../struct.FixedI128.html) with 30 integer bits and 98 fractional bits.
pub type I30F98 = FixedI128<frac::U98>;
/// [`FixedI128`](../struct.FixedI128.html) with 29 integer bits and 99 fractional bits.
pub type I29F99 = FixedI128<frac::U99>;
/// [`FixedI128`](../struct.FixedI128.html) with 28 integer bits and 100 fractional bits.
pub type I28F100 = FixedI128<frac::U100>;
/// [`FixedI128`](../struct.FixedI128.html) with 27 integer bits and 101 fractional bits.
pub type I27F101 = FixedI128<frac::U101>;
/// [`FixedI128`](../struct.FixedI128.html) with 26 integer bits and 102 fractional bits.
pub type I26F102 = FixedI128<frac::U102>;
/// [`FixedI128`](../struct.FixedI128.html) with 25 integer bits and 103 fractional bits.
pub type I25F103 = FixedI128<frac::U103>;
/// [`FixedI128`](../struct.FixedI128.html) with 24 integer bits and 104 fractional bits.
pub type I24F104 = FixedI128<frac::U104>;
/// [`FixedI128`](../struct.FixedI128.html) with 23 integer bits and 105 fractional bits.
pub type I23F105 = FixedI128<frac::U105>;
/// [`FixedI128`](../struct.FixedI128.html) with 22 integer bits and 106 fractional bits.
pub type I22F106 = FixedI128<frac::U106>;
/// [`FixedI128`](../struct.FixedI128.html) with 21 integer bits and 107 fractional bits.
pub type I21F107 = FixedI128<frac::U107>;
/// [`FixedI128`](../struct.FixedI128.html) with 20 integer bits and 108 fractional bits.
pub type I20F108 = FixedI128<frac::U108>;
/// [`FixedI128`](../struct.FixedI128.html) with 19 integer bits and 109 fractional bits.
pub type I19F109 = FixedI128<frac::U109>;
/// [`FixedI128`](../struct.FixedI128.html) with 18 integer bits and 110 fractional bits.
pub type I18F110 = FixedI128<frac::U110>;
/// [`FixedI128`](../struct.FixedI128.html) with 17 integer bits and 111 fractional bits.
pub type I17F111 = FixedI128<frac::U111>;
/// [`FixedI128`](../struct.FixedI128.html) with 16 integer bits and 112 fractional bits.
pub type I16F112 = FixedI128<frac::U112>;
/// [`FixedI128`](../struct.FixedI128.html) with 15 integer bits and 113 fractional bits.
pub type I15F113 = FixedI128<frac::U113>;
/// [`FixedI128`](../struct.FixedI128.html) with 14 integer bits and 114 fractional bits.
pub type I14F114 = FixedI128<frac::U114>;
/// [`FixedI128`](../struct.FixedI128.html) with 13 integer bits and 115 fractional bits.
pub type I13F115 = FixedI128<frac::U115>;
/// [`FixedI128`](../struct.FixedI128.html) with 12 integer bits and 116 fractional bits.
pub type I12F116 = FixedI128<frac::U116>;
/// [`FixedI128`](../struct.FixedI128.html) with 11 integer bits and 117 fractional bits.
pub type I11F117 = FixedI128<frac::U117>;
/// [`FixedI128`](../struct.FixedI128.html) with 10 integer bits and 118 fractional bits.
pub type I10F118 = FixedI128<frac::U118>;
/// [`FixedI128`](../struct.FixedI128.html) with nine integer bits and 119 fractional bits.
pub type I9F119 = FixedI128<frac::U119>;
/// [`FixedI128`](../struct.FixedI128.html) with eight integer bits and 120 fractional bits.
pub type I8F120 = FixedI128<frac::U120>;
/// [`FixedI128`](../struct.FixedI128.html) with seven integer bits and 121 fractional bits.
pub type I7F121 = FixedI128<frac::U121>;
/// [`FixedI128`](../struct.FixedI128.html) with six integer bits and 122 fractional bits.
pub type I6F122 = FixedI128<frac::U122>;
/// [`FixedI128`](../struct.FixedI128.html) with five integer bits and 123 fractional bits.
pub type I5F123 = FixedI128<frac::U123>;
/// [`FixedI128`](../struct.FixedI128.html) with four integer bits and 124 fractional bits.
pub type I4F124 = FixedI128<frac::U124>;
/// [`FixedI128`](../struct.FixedI128.html) with three integer bits and 125 fractional bits.
pub type I3F125 = FixedI128<frac::U125>;
/// [`FixedI128`](../struct.FixedI128.html) with two integer bits and 126 fractional bits.
pub type I2F126 = FixedI128<frac::U126>;
/// [`FixedI128`](../struct.FixedI128.html) with one integer bit and 127 fractional bits.
pub type I1F127 = FixedI128<frac::U127>;
/// [`FixedI128`](../struct.FixedI128.html) with no integer bits and 128 fractional bits.
pub type I0F128 = FixedI128<frac::U128>;
/// [`FixedU8`](../struct.FixedU8.html) with eight integer bits and no fractional bits.
pub type U8F0 = FixedU8<frac::U0>;
/// [`FixedU8`](../struct.FixedU8.html) with seven integer bits and one fractional bit.
pub type U7F1 = FixedU8<frac::U1>;
/// [`FixedU8`](../struct.FixedU8.html) with six integer bits and two fractional bits.
pub type U6F2 = FixedU8<frac::U2>;
/// [`FixedU8`](../struct.FixedU8.html) with five integer bits and three fractional bits.
pub type U5F3 = FixedU8<frac::U3>;
/// [`FixedU8`](../struct.FixedU8.html) with four integer bits and four fractional bits.
pub type U4F4 = FixedU8<frac::U4>;
/// [`FixedU8`](../struct.FixedU8.html) with three integer bits and five fractional bits.
pub type U3F5 = FixedU8<frac::U5>;
/// [`FixedU8`](../struct.FixedU8.html) with two integer bits and six fractional bits.
pub type U2F6 = FixedU8<frac::U6>;
/// [`FixedU8`](../struct.FixedU8.html) with one integer bit and seven fractional bits.
pub type U1F7 = FixedU8<frac::U7>;
/// [`FixedU8`](../struct.FixedU8.html) with no integer bits and eight fractional bits.
pub type U0F8 = FixedU8<frac::U8>;
/// [`FixedU16`](../struct.FixedU16.html) with 16 integer bits and no fractional bits.
pub type U16F0 = FixedU16<frac::U0>;
/// [`FixedU16`](../struct.FixedU16.html) with 15 integer bits and one fractional bit.
pub type U15F1 = FixedU16<frac::U1>;
/// [`FixedU16`](../struct.FixedU16.html) with 14 integer bits and two fractional bits.
pub type U14F2 = FixedU16<frac::U2>;
/// [`FixedU16`](../struct.FixedU16.html) with 13 integer bits and three fractional bits.
pub type U13F3 = FixedU16<frac::U3>;
/// [`FixedU16`](../struct.FixedU16.html) with 12 integer bits and four fractional bits.
pub type U12F4 = FixedU16<frac::U4>;
/// [`FixedU16`](../struct.FixedU16.html) with 11 integer bits and five fractional bits.
pub type U11F5 = FixedU16<frac::U5>;
/// [`FixedU16`](../struct.FixedU16.html) with 10 integer bits and six fractional bits.
pub type U10F6 = FixedU16<frac::U6>;
/// [`FixedU16`](../struct.FixedU16.html) with nine integer bits and seven fractional bits.
pub type U9F7 = FixedU16<frac::U7>;
/// [`FixedU16`](../struct.FixedU16.html) with eight integer bits and eight fractional bits.
pub type U8F8 = FixedU16<frac::U8>;
/// [`FixedU16`](../struct.FixedU16.html) with seven integer bits and nine fractional bits.
pub type U7F9 = FixedU16<frac::U9>;
/// [`FixedU16`](../struct.FixedU16.html) with six integer bits and 10 fractional bits.
pub type U6F10 = FixedU16<frac::U10>;
/// [`FixedU16`](../struct.FixedU16.html) with five integer bits and 11 fractional bits.
pub type U5F11 = FixedU16<frac::U11>;
/// [`FixedU16`](../struct.FixedU16.html) with four integer bits and 12 fractional bits.
pub type U4F12 = FixedU16<frac::U12>;
/// [`FixedU16`](../struct.FixedU16.html) with three integer bits and 13 fractional bits.
pub type U3F13 = FixedU16<frac::U13>;
/// [`FixedU16`](../struct.FixedU16.html) with two integer bits and 14 fractional bits.
pub type U2F14 = FixedU16<frac::U14>;
/// [`FixedU16`](../struct.FixedU16.html) with one integer bit and 15 fractional bits.
pub type U1F15 = FixedU16<frac::U15>;
/// [`FixedU16`](../struct.FixedU16.html) with no integer bits and 16 fractional bits.
pub type U0F16 = FixedU16<frac::U16>;
/// [`FixedU32`](../struct.FixedU32.html) with 32 integer bits and no fractional bits.
pub type U32F0 = FixedU32<frac::U0>;
/// [`FixedU32`](../struct.FixedU32.html) with 31 integer bits and one fractional bit.
pub type U31F1 = FixedU32<frac::U1>;
/// [`FixedU32`](../struct.FixedU32.html) with 30 integer bits and two fractional bits.
pub type U30F2 = FixedU32<frac::U2>;
/// [`FixedU32`](../struct.FixedU32.html) with 29 integer bits and three fractional bits.
pub type U29F3 = FixedU32<frac::U3>;
/// [`FixedU32`](../struct.FixedU32.html) with 28 integer bits and four fractional bits.
pub type U28F4 = FixedU32<frac::U4>;
/// [`FixedU32`](../struct.FixedU32.html) with 27 integer bits and five fractional bits.
pub type U27F5 = FixedU32<frac::U5>;
/// [`FixedU32`](../struct.FixedU32.html) with 26 integer bits and six fractional bits.
pub type U26F6 = FixedU32<frac::U6>;
/// [`FixedU32`](../struct.FixedU32.html) with 25 integer bits and seven fractional bits.
pub type U25F7 = FixedU32<frac::U7>;
/// [`FixedU32`](../struct.FixedU32.html) with 24 integer bits and eight fractional bits.
pub type U24F8 = FixedU32<frac::U8>;
/// [`FixedU32`](../struct.FixedU32.html) with 23 integer bits and nine fractional bits.
pub type U23F9 = FixedU32<frac::U9>;
/// [`FixedU32`](../struct.FixedU32.html) with 22 integer bits and 10 fractional bits.
pub type U22F10 = FixedU32<frac::U10>;
/// [`FixedU32`](../struct.FixedU32.html) with 21 integer bits and 11 fractional bits.
pub type U21F11 = FixedU32<frac::U11>;
/// [`FixedU32`](../struct.FixedU32.html) with 20 integer bits and 12 fractional bits.
pub type U20F12 = FixedU32<frac::U12>;
/// [`FixedU32`](../struct.FixedU32.html) with 19 integer bits and 13 fractional bits.
pub type U19F13 = FixedU32<frac::U13>;
/// [`FixedU32`](../struct.FixedU32.html) with 18 integer bits and 14 fractional bits.
pub type U18F14 = FixedU32<frac::U14>;
/// [`FixedU32`](../struct.FixedU32.html) with 17 integer bits and 15 fractional bits.
pub type U17F15 = FixedU32<frac::U15>;
/// [`FixedU32`](../struct.FixedU32.html) with 16 integer bits and 16 fractional bits.
pub type U16F16 = FixedU32<frac::U16>;
/// [`FixedU32`](../struct.FixedU32.html) with 15 integer bits and 17 fractional bits.
pub type U15F17 = FixedU32<frac::U17>;
/// [`FixedU32`](../struct.FixedU32.html) with 14 integer bits and 18 fractional bits.
pub type U14F18 = FixedU32<frac::U18>;
/// [`FixedU32`](../struct.FixedU32.html) with 13 integer bits and 19 fractional bits.
pub type U13F19 = FixedU32<frac::U19>;
/// [`FixedU32`](../struct.FixedU32.html) with 12 integer bits and 20 fractional bits.
pub type U12F20 = FixedU32<frac::U20>;
/// [`FixedU32`](../struct.FixedU32.html) with 11 integer bits and 21 fractional bits.
pub type U11F21 = FixedU32<frac::U21>;
/// [`FixedU32`](../struct.FixedU32.html) with 10 integer bits and 22 fractional bits.
pub type U10F22 = FixedU32<frac::U22>;
/// [`FixedU32`](../struct.FixedU32.html) with nine integer bits and 23 fractional bits.
pub type U9F23 = FixedU32<frac::U23>;
/// [`FixedU32`](../struct.FixedU32.html) with eight integer bits and 24 fractional bits.
pub type U8F24 = FixedU32<frac::U24>;
/// [`FixedU32`](../struct.FixedU32.html) with seven integer bits and 25 fractional bits.
pub type U7F25 = FixedU32<frac::U25>;
/// [`FixedU32`](../struct.FixedU32.html) with six integer bits and 26 fractional bits.
pub type U6F26 = FixedU32<frac::U26>;
/// [`FixedU32`](../struct.FixedU32.html) with five integer bits and 27 fractional bits.
pub type U5F27 = FixedU32<frac::U27>;
/// [`FixedU32`](../struct.FixedU32.html) with four integer bits and 28 fractional bits.
pub type U4F28 = FixedU32<frac::U28>;
/// [`FixedU32`](../struct.FixedU32.html) with three integer bits and 29 fractional bits.
pub type U3F29 = FixedU32<frac::U29>;
/// [`FixedU32`](../struct.FixedU32.html) with two integer bits and 30 fractional bits.
pub type U2F30 = FixedU32<frac::U30>;
/// [`FixedU32`](../struct.FixedU32.html) with one integer bit and 31 fractional bits.
pub type U1F31 = FixedU32<frac::U31>;
/// [`FixedU32`](../struct.FixedU32.html) with no integer bits and 32 fractional bits.
pub type U0F32 = FixedU32<frac::U32>;
/// [`FixedU64`](../struct.FixedU64.html) with 64 integer bits and no fractional bits.
pub type U64F0 = FixedU64<frac::U0>;
/// [`FixedU64`](../struct.FixedU64.html) with 63 integer bits and one fractional bit.
pub type U63F1 = FixedU64<frac::U1>;
/// [`FixedU64`](../struct.FixedU64.html) with 62 integer bits and two fractional bits.
pub type U62F2 = FixedU64<frac::U2>;
/// [`FixedU64`](../struct.FixedU64.html) with 61 integer bits and three fractional bits.
pub type U61F3 = FixedU64<frac::U3>;
/// [`FixedU64`](../struct.FixedU64.html) with 60 integer bits and four fractional bits.
pub type U60F4 = FixedU64<frac::U4>;
/// [`FixedU64`](../struct.FixedU64.html) with 59 integer bits and five fractional bits.
pub type U59F5 = FixedU64<frac::U5>;
/// [`FixedU64`](../struct.FixedU64.html) with 58 integer bits and six fractional bits.
pub type U58F6 = FixedU64<frac::U6>;
/// [`FixedU64`](../struct.FixedU64.html) with 57 integer bits and seven fractional bits.
pub type U57F7 = FixedU64<frac::U7>;
/// [`FixedU64`](../struct.FixedU64.html) with 56 integer bits and eight fractional bits.
pub type U56F8 = FixedU64<frac::U8>;
/// [`FixedU64`](../struct.FixedU64.html) with 55 integer bits and nine fractional bits.
pub type U55F9 = FixedU64<frac::U9>;
/// [`FixedU64`](../struct.FixedU64.html) with 54 integer bits and 10 fractional bits.
pub type U54F10 = FixedU64<frac::U10>;
/// [`FixedU64`](../struct.FixedU64.html) with 53 integer bits and 11 fractional bits.
pub type U53F11 = FixedU64<frac::U11>;
/// [`FixedU64`](../struct.FixedU64.html) with 52 integer bits and 12 fractional bits.
pub type U52F12 = FixedU64<frac::U12>;
/// [`FixedU64`](../struct.FixedU64.html) with 51 integer bits and 13 fractional bits.
pub type U51F13 = FixedU64<frac::U13>;
/// [`FixedU64`](../struct.FixedU64.html) with 50 integer bits and 14 fractional bits.
pub type U50F14 = FixedU64<frac::U14>;
/// [`FixedU64`](../struct.FixedU64.html) with 49 integer bits and 15 fractional bits.
pub type U49F15 = FixedU64<frac::U15>;
/// [`FixedU64`](../struct.FixedU64.html) with 48 integer bits and 16 fractional bits.
pub type U48F16 = FixedU64<frac::U16>;
/// [`FixedU64`](../struct.FixedU64.html) with 47 integer bits and 17 fractional bits.
pub type U47F17 = FixedU64<frac::U17>;
/// [`FixedU64`](../struct.FixedU64.html) with 46 integer bits and 18 fractional bits.
pub type U46F18 = FixedU64<frac::U18>;
/// [`FixedU64`](../struct.FixedU64.html) with 45 integer bits and 19 fractional bits.
pub type U45F19 = FixedU64<frac::U19>;
/// [`FixedU64`](../struct.FixedU64.html) with 44 integer bits and 20 fractional bits.
pub type U44F20 = FixedU64<frac::U20>;
/// [`FixedU64`](../struct.FixedU64.html) with 43 integer bits and 21 fractional bits.
pub type U43F21 = FixedU64<frac::U21>;
/// [`FixedU64`](../struct.FixedU64.html) with 42 integer bits and 22 fractional bits.
pub type U42F22 = FixedU64<frac::U22>;
/// [`FixedU64`](../struct.FixedU64.html) with 41 integer bits and 23 fractional bits.
pub type U41F23 = FixedU64<frac::U23>;
/// [`FixedU64`](../struct.FixedU64.html) with 40 integer bits and 24 fractional bits.
pub type U40F24 = FixedU64<frac::U24>;
/// [`FixedU64`](../struct.FixedU64.html) with 39 integer bits and 25 fractional bits.
pub type U39F25 = FixedU64<frac::U25>;
/// [`FixedU64`](../struct.FixedU64.html) with 38 integer bits and 26 fractional bits.
pub type U38F26 = FixedU64<frac::U26>;
/// [`FixedU64`](../struct.FixedU64.html) with 37 integer bits and 27 fractional bits.
pub type U37F27 = FixedU64<frac::U27>;
/// [`FixedU64`](../struct.FixedU64.html) with 36 integer bits and 28 fractional bits.
pub type U36F28 = FixedU64<frac::U28>;
/// [`FixedU64`](../struct.FixedU64.html) with 35 integer bits and 29 fractional bits.
pub type U35F29 = FixedU64<frac::U29>;
/// [`FixedU64`](../struct.FixedU64.html) with 34 integer bits and 30 fractional bits.
pub type U34F30 = FixedU64<frac::U30>;
/// [`FixedU64`](../struct.FixedU64.html) with 33 integer bits and 31 fractional bits.
pub type U33F31 = FixedU64<frac::U31>;
/// [`FixedU64`](../struct.FixedU64.html) with 32 integer bits and 32 fractional bits.
pub type U32F32 = FixedU64<frac::U32>;
/// [`FixedU64`](../struct.FixedU64.html) with 31 integer bits and 33 fractional bits.
pub type U31F33 = FixedU64<frac::U33>;
/// [`FixedU64`](../struct.FixedU64.html) with 30 integer bits and 34 fractional bits.
pub type U30F34 = FixedU64<frac::U34>;
/// [`FixedU64`](../struct.FixedU64.html) with 29 integer bits and 35 fractional bits.
pub type U29F35 = FixedU64<frac::U35>;
/// [`FixedU64`](../struct.FixedU64.html) with 28 integer bits and 36 fractional bits.
pub type U28F36 = FixedU64<frac::U36>;
/// [`FixedU64`](../struct.FixedU64.html) with 27 integer bits and 37 fractional bits.
pub type U27F37 = FixedU64<frac::U37>;
/// [`FixedU64`](../struct.FixedU64.html) with 26 integer bits and 38 fractional bits.
pub type U26F38 = FixedU64<frac::U38>;
/// [`FixedU64`](../struct.FixedU64.html) with 25 integer bits and 39 fractional bits.
pub type U25F39 = FixedU64<frac::U39>;
/// [`FixedU64`](../struct.FixedU64.html) with 24 integer bits and 40 fractional bits.
pub type U24F40 = FixedU64<frac::U40>;
/// [`FixedU64`](../struct.FixedU64.html) with 23 integer bits and 41 fractional bits.
pub type U23F41 = FixedU64<frac::U41>;
/// [`FixedU64`](../struct.FixedU64.html) with 22 integer bits and 42 fractional bits.
pub type U22F42 = FixedU64<frac::U42>;
/// [`FixedU64`](../struct.FixedU64.html) with 21 integer bits and 43 fractional bits.
pub type U21F43 = FixedU64<frac::U43>;
/// [`FixedU64`](../struct.FixedU64.html) with 20 integer bits and 44 fractional bits.
pub type U20F44 = FixedU64<frac::U44>;
/// [`FixedU64`](../struct.FixedU64.html) with 19 integer bits and 45 fractional bits.
pub type U19F45 = FixedU64<frac::U45>;
/// [`FixedU64`](../struct.FixedU64.html) with 18 integer bits and 46 fractional bits.
pub type U18F46 = FixedU64<frac::U46>;
/// [`FixedU64`](../struct.FixedU64.html) with 17 integer bits and 47 fractional bits.
pub type U17F47 = FixedU64<frac::U47>;
/// [`FixedU64`](../struct.FixedU64.html) with 16 integer bits and 48 fractional bits.
pub type U16F48 = FixedU64<frac::U48>;
/// [`FixedU64`](../struct.FixedU64.html) with 15 integer bits and 49 fractional bits.
pub type U15F49 = FixedU64<frac::U49>;
/// [`FixedU64`](../struct.FixedU64.html) with 14 integer bits and 50 fractional bits.
pub type U14F50 = FixedU64<frac::U50>;
/// [`FixedU64`](../struct.FixedU64.html) with 13 integer bits and 51 fractional bits.
pub type U13F51 = FixedU64<frac::U51>;
/// [`FixedU64`](../struct.FixedU64.html) with 12 integer bits and 52 fractional bits.
pub type U12F52 = FixedU64<frac::U52>;
/// [`FixedU64`](../struct.FixedU64.html) with 11 integer bits and 53 fractional bits.
pub type U11F53 = FixedU64<frac::U53>;
/// [`FixedU64`](../struct.FixedU64.html) with 10 integer bits and 54 fractional bits.
pub type U10F54 = FixedU64<frac::U54>;
/// [`FixedU64`](../struct.FixedU64.html) with nine integer bits and 55 fractional bits.
pub type U9F55 = FixedU64<frac::U55>;
/// [`FixedU64`](../struct.FixedU64.html) with eight integer bits and 56 fractional bits.
pub type U8F56 = FixedU64<frac::U56>;
/// [`FixedU64`](../struct.FixedU64.html) with seven integer bits and 57 fractional bits.
pub type U7F57 = FixedU64<frac::U57>;
/// [`FixedU64`](../struct.FixedU64.html) with six integer bits and 58 fractional bits.
pub type U6F58 = FixedU64<frac::U58>;
/// [`FixedU64`](../struct.FixedU64.html) with five integer bits and 59 fractional bits.
pub type U5F59 = FixedU64<frac::U59>;
/// [`FixedU64`](../struct.FixedU64.html) with four integer bits and 60 fractional bits.
pub type U4F60 = FixedU64<frac::U60>;
/// [`FixedU64`](../struct.FixedU64.html) with three integer bits and 61 fractional bits.
pub type U3F61 = FixedU64<frac::U61>;
/// [`FixedU64`](../struct.FixedU64.html) with two integer bits and 62 fractional bits.
pub type U2F62 = FixedU64<frac::U62>;
/// [`FixedU64`](../struct.FixedU64.html) with one integer bit and 63 fractional bits.
pub type U1F63 = FixedU64<frac::U63>;
/// [`FixedU64`](../struct.FixedU64.html) with no integer bits and 64 fractional bits.
pub type U0F64 = FixedU64<frac::U64>;
/// [`FixedU128`](../struct.FixedU128.html) with 128 integer bits and no fractional bits.
pub type U128F0 = FixedU128<frac::U0>;
/// [`FixedU128`](../struct.FixedU128.html) with 127 integer bits and one fractional bit.
pub type U127F1 = FixedU128<frac::U1>;
/// [`FixedU128`](../struct.FixedU128.html) with 126 integer bits and two fractional bits.
pub type U126F2 = FixedU128<frac::U2>;
/// [`FixedU128`](../struct.FixedU128.html) with 125 integer bits and three fractional bits.
pub type U125F3 = FixedU128<frac::U3>;
/// [`FixedU128`](../struct.FixedU128.html) with 124 integer bits and four fractional bits.
pub type U124F4 = FixedU128<frac::U4>;
/// [`FixedU128`](../struct.FixedU128.html) with 123 integer bits and five fractional bits.
pub type U123F5 = FixedU128<frac::U5>;
/// [`FixedU128`](../struct.FixedU128.html) with 122 integer bits and six fractional bits.
pub type U122F6 = FixedU128<frac::U6>;
/// [`FixedU128`](../struct.FixedU128.html) with 121 integer bits and seven fractional bits.
pub type U121F7 = FixedU128<frac::U7>;
/// [`FixedU128`](../struct.FixedU128.html) with 120 integer bits and eight fractional bits.
pub type U120F8 = FixedU128<frac::U8>;
/// [`FixedU128`](../struct.FixedU128.html) with 119 integer bits and nine fractional bits.
pub type U119F9 = FixedU128<frac::U9>;
/// [`FixedU128`](../struct.FixedU128.html) with 118 integer bits and 10 fractional bits.
pub type U118F10 = FixedU128<frac::U10>;
/// [`FixedU128`](../struct.FixedU128.html) with 117 integer bits and 11 fractional bits.
pub type U117F11 = FixedU128<frac::U11>;
/// [`FixedU128`](../struct.FixedU128.html) with 116 integer bits and 12 fractional bits.
pub type U116F12 = FixedU128<frac::U12>;
/// [`FixedU128`](../struct.FixedU128.html) with 115 integer bits and 13 fractional bits.
pub type U115F13 = FixedU128<frac::U13>;
/// [`FixedU128`](../struct.FixedU128.html) with 114 integer bits and 14 fractional bits.
pub type U114F14 = FixedU128<frac::U14>;
/// [`FixedU128`](../struct.FixedU128.html) with 113 integer bits and 15 fractional bits.
pub type U113F15 = FixedU128<frac::U15>;
/// [`FixedU128`](../struct.FixedU128.html) with 112 integer bits and 16 fractional bits.
pub type U112F16 = FixedU128<frac::U16>;
/// [`FixedU128`](../struct.FixedU128.html) with 111 integer bits and 17 fractional bits.
pub type U111F17 = FixedU128<frac::U17>;
/// [`FixedU128`](../struct.FixedU128.html) with 110 integer bits and 18 fractional bits.
pub type U110F18 = FixedU128<frac::U18>;
/// [`FixedU128`](../struct.FixedU128.html) with 109 integer bits and 19 fractional bits.
pub type U109F19 = FixedU128<frac::U19>;
/// [`FixedU128`](../struct.FixedU128.html) with 108 integer bits and 20 fractional bits.
pub type U108F20 = FixedU128<frac::U20>;
/// [`FixedU128`](../struct.FixedU128.html) with 107 integer bits and 21 fractional bits.
pub type U107F21 = FixedU128<frac::U21>;
/// [`FixedU128`](../struct.FixedU128.html) with 106 integer bits and 22 fractional bits.
pub type U106F22 = FixedU128<frac::U22>;
/// [`FixedU128`](../struct.FixedU128.html) with 105 integer bits and 23 fractional bits.
pub type U105F23 = FixedU128<frac::U23>;
/// [`FixedU128`](../struct.FixedU128.html) with 104 integer bits and 24 fractional bits.
pub type U104F24 = FixedU128<frac::U24>;
/// [`FixedU128`](../struct.FixedU128.html) with 103 integer bits and 25 fractional bits.
pub type U103F25 = FixedU128<frac::U25>;
/// [`FixedU128`](../struct.FixedU128.html) with 102 integer bits and 26 fractional bits.
pub type U102F26 = FixedU128<frac::U26>;
/// [`FixedU128`](../struct.FixedU128.html) with 101 integer bits and 27 fractional bits.
pub type U101F27 = FixedU128<frac::U27>;
/// [`FixedU128`](../struct.FixedU128.html) with 100 integer bits and 28 fractional bits.
pub type U100F28 = FixedU128<frac::U28>;
/// [`FixedU128`](../struct.FixedU128.html) with 99 integer bits and 29 fractional bits.
pub type U99F29 = FixedU128<frac::U29>;
/// [`FixedU128`](../struct.FixedU128.html) with 98 integer bits and 30 fractional bits.
pub type U98F30 = FixedU128<frac::U30>;
/// [`FixedU128`](../struct.FixedU128.html) with 97 integer bits and 31 fractional bits.
pub type U97F31 = FixedU128<frac::U31>;
/// [`FixedU128`](../struct.FixedU128.html) with 96 integer bits and 32 fractional bits.
pub type U96F32 = FixedU128<frac::U32>;
/// [`FixedU128`](../struct.FixedU128.html) with 95 integer bits and 33 fractional bits.
pub type U95F33 = FixedU128<frac::U33>;
/// [`FixedU128`](../struct.FixedU128.html) with 94 integer bits and 34 fractional bits.
pub type U94F34 = FixedU128<frac::U34>;
/// [`FixedU128`](../struct.FixedU128.html) with 93 integer bits and 35 fractional bits.
pub type U93F35 = FixedU128<frac::U35>;
/// [`FixedU128`](../struct.FixedU128.html) with 92 integer bits and 36 fractional bits.
pub type U92F36 = FixedU128<frac::U36>;
/// [`FixedU128`](../struct.FixedU128.html) with 91 integer bits and 37 fractional bits.
pub type U91F37 = FixedU128<frac::U37>;
/// [`FixedU128`](../struct.FixedU128.html) with 90 integer bits and 38 fractional bits.
pub type U90F38 = FixedU128<frac::U38>;
/// [`FixedU128`](../struct.FixedU128.html) with 89 integer bits and 39 fractional bits.
pub type U89F39 = FixedU128<frac::U39>;
/// [`FixedU128`](../struct.FixedU128.html) with 88 integer bits and 40 fractional bits.
pub type U88F40 = FixedU128<frac::U40>;
/// [`FixedU128`](../struct.FixedU128.html) with 87 integer bits and 41 fractional bits.
pub type U87F41 = FixedU128<frac::U41>;
/// [`FixedU128`](../struct.FixedU128.html) with 86 integer bits and 42 fractional bits.
pub type U86F42 = FixedU128<frac::U42>;
/// [`FixedU128`](../struct.FixedU128.html) with 85 integer bits and 43 fractional bits.
pub type U85F43 = FixedU128<frac::U43>;
/// [`FixedU128`](../struct.FixedU128.html) with 84 integer bits and 44 fractional bits.
pub type U84F44 = FixedU128<frac::U44>;
/// [`FixedU128`](../struct.FixedU128.html) with 83 integer bits and 45 fractional bits.
pub type U83F45 = FixedU128<frac::U45>;
/// [`FixedU128`](../struct.FixedU128.html) with 82 integer bits and 46 fractional bits.
pub type U82F46 = FixedU128<frac::U46>;
/// [`FixedU128`](../struct.FixedU128.html) with 81 integer bits and 47 fractional bits.
pub type U81F47 = FixedU128<frac::U47>;
/// [`FixedU128`](../struct.FixedU128.html) with 80 integer bits and 48 fractional bits.
pub type U80F48 = FixedU128<frac::U48>;
/// [`FixedU128`](../struct.FixedU128.html) with 79 integer bits and 49 fractional bits.
pub type U79F49 = FixedU128<frac::U49>;
/// [`FixedU128`](../struct.FixedU128.html) with 78 integer bits and 50 fractional bits.
pub type U78F50 = FixedU128<frac::U50>;
/// [`FixedU128`](../struct.FixedU128.html) with 77 integer bits and 51 fractional bits.
pub type U77F51 = FixedU128<frac::U51>;
/// [`FixedU128`](../struct.FixedU128.html) with 76 integer bits and 52 fractional bits.
pub type U76F52 = FixedU128<frac::U52>;
/// [`FixedU128`](../struct.FixedU128.html) with 75 integer bits and 53 fractional bits.
pub type U75F53 = FixedU128<frac::U53>;
/// [`FixedU128`](../struct.FixedU128.html) with 74 integer bits and 54 fractional bits.
pub type U74F54 = FixedU128<frac::U54>;
/// [`FixedU128`](../struct.FixedU128.html) with 73 integer bits and 55 fractional bits.
pub type U73F55 = FixedU128<frac::U55>;
/// [`FixedU128`](../struct.FixedU128.html) with 72 integer bits and 56 fractional bits.
pub type U72F56 = FixedU128<frac::U56>;
/// [`FixedU128`](../struct.FixedU128.html) with 71 integer bits and 57 fractional bits.
pub type U71F57 = FixedU128<frac::U57>;
/// [`FixedU128`](../struct.FixedU128.html) with 70 integer bits and 58 fractional bits.
pub type U70F58 = FixedU128<frac::U58>;
/// [`FixedU128`](../struct.FixedU128.html) with 69 integer bits and 59 fractional bits.
pub type U69F59 = FixedU128<frac::U59>;
/// [`FixedU128`](../struct.FixedU128.html) with 68 integer bits and 60 fractional bits.
pub type U68F60 = FixedU128<frac::U60>;
/// [`FixedU128`](../struct.FixedU128.html) with 67 integer bits and 61 fractional bits.
pub type U67F61 = FixedU128<frac::U61>;
/// [`FixedU128`](../struct.FixedU128.html) with 66 integer bits and 62 fractional bits.
pub type U66F62 = FixedU128<frac::U62>;
/// [`FixedU128`](../struct.FixedU128.html) with 65 integer bits and 63 fractional bits.
pub type U65F63 = FixedU128<frac::U63>;
/// [`FixedU128`](../struct.FixedU128.html) with 64 integer bits and 64 fractional bits.
pub type U64F64 = FixedU128<frac::U64>;
/// [`FixedU128`](../struct.FixedU128.html) with 63 integer bits and 65 fractional bits.
pub type U63F65 = FixedU128<frac::U65>;
/// [`FixedU128`](../struct.FixedU128.html) with 62 integer bits and 66 fractional bits.
pub type U62F66 = FixedU128<frac::U66>;
/// [`FixedU128`](../struct.FixedU128.html) with 61 integer bits and 67 fractional bits.
pub type U61F67 = FixedU128<frac::U67>;
/// [`FixedU128`](../struct.FixedU128.html) with 60 integer bits and 68 fractional bits.
pub type U60F68 = FixedU128<frac::U68>;
/// [`FixedU128`](../struct.FixedU128.html) with 59 integer bits and 69 fractional bits.
pub type U59F69 = FixedU128<frac::U69>;
/// [`FixedU128`](../struct.FixedU128.html) with 58 integer bits and 70 fractional bits.
pub type U58F70 = FixedU128<frac::U70>;
/// [`FixedU128`](../struct.FixedU128.html) with 57 integer bits and 71 fractional bits.
pub type U57F71 = FixedU128<frac::U71>;
/// [`FixedU128`](../struct.FixedU128.html) with 56 integer bits and 72 fractional bits.
pub type U56F72 = FixedU128<frac::U72>;
/// [`FixedU128`](../struct.FixedU128.html) with 55 integer bits and 73 fractional bits.
pub type U55F73 = FixedU128<frac::U73>;
/// [`FixedU128`](../struct.FixedU128.html) with 54 integer bits and 74 fractional bits.
pub type U54F74 = FixedU128<frac::U74>;
/// [`FixedU128`](../struct.FixedU128.html) with 53 integer bits and 75 fractional bits.
pub type U53F75 = FixedU128<frac::U75>;
/// [`FixedU128`](../struct.FixedU128.html) with 52 integer bits and 76 fractional bits.
pub type U52F76 = FixedU128<frac::U76>;
/// [`FixedU128`](../struct.FixedU128.html) with 51 integer bits and 77 fractional bits.
pub type U51F77 = FixedU128<frac::U77>;
/// [`FixedU128`](../struct.FixedU128.html) with 50 integer bits and 78 fractional bits.
pub type U50F78 = FixedU128<frac::U78>;
/// [`FixedU128`](../struct.FixedU128.html) with 49 integer bits and 79 fractional bits.
pub type U49F79 = FixedU128<frac::U79>;
/// [`FixedU128`](../struct.FixedU128.html) with 48 integer bits and 80 fractional bits.
pub type U48F80 = FixedU128<frac::U80>;
/// [`FixedU128`](../struct.FixedU128.html) with 47 integer bits and 81 fractional bits.
pub type U47F81 = FixedU128<frac::U81>;
/// [`FixedU128`](../struct.FixedU128.html) with 46 integer bits and 82 fractional bits.
pub type U46F82 = FixedU128<frac::U82>;
/// [`FixedU128`](../struct.FixedU128.html) with 45 integer bits and 83 fractional bits.
pub type U45F83 = FixedU128<frac::U83>;
/// [`FixedU128`](../struct.FixedU128.html) with 44 integer bits and 84 fractional bits.
pub type U44F84 = FixedU128<frac::U84>;
/// [`FixedU128`](../struct.FixedU128.html) with 43 integer bits and 85 fractional bits.
pub type U43F85 = FixedU128<frac::U85>;
/// [`FixedU128`](../struct.FixedU128.html) with 42 integer bits and 86 fractional bits.
pub type U42F86 = FixedU128<frac::U86>;
/// [`FixedU128`](../struct.FixedU128.html) with 41 integer bits and 87 fractional bits.
pub type U41F87 = FixedU128<frac::U87>;
/// [`FixedU128`](../struct.FixedU128.html) with 40 integer bits and 88 fractional bits.
pub type U40F88 = FixedU128<frac::U88>;
/// [`FixedU128`](../struct.FixedU128.html) with 39 integer bits and 89 fractional bits.
pub type U39F89 = FixedU128<frac::U89>;
/// [`FixedU128`](../struct.FixedU128.html) with 38 integer bits and 90 fractional bits.
pub type U38F90 = FixedU128<frac::U90>;
/// [`FixedU128`](../struct.FixedU128.html) with 37 integer bits and 91 fractional bits.
pub type U37F91 = FixedU128<frac::U91>;
/// [`FixedU128`](../struct.FixedU128.html) with 36 integer bits and 92 fractional bits.
pub type U36F92 = FixedU128<frac::U92>;
/// [`FixedU128`](../struct.FixedU128.html) with 35 integer bits and 93 fractional bits.
pub type U35F93 = FixedU128<frac::U93>;
/// [`FixedU128`](../struct.FixedU128.html) with 34 integer bits and 94 fractional bits.
pub type U34F94 = FixedU128<frac::U94>;
/// [`FixedU128`](../struct.FixedU128.html) with 33 integer bits and 95 fractional bits.
pub type U33F95 = FixedU128<frac::U95>;
/// [`FixedU128`](../struct.FixedU128.html) with 32 integer bits and 96 fractional bits.
pub type U32F96 = FixedU128<frac::U96>;
/// [`FixedU128`](../struct.FixedU128.html) with 31 integer bits and 97 fractional bits.
pub type U31F97 = FixedU128<frac::U97>;
/// [`FixedU128`](../struct.FixedU128.html) with 30 integer bits and 98 fractional bits.
pub type U30F98 = FixedU128<frac::U98>;
/// [`FixedU128`](../struct.FixedU128.html) with 29 integer bits and 99 fractional bits.
pub type U29F99 = FixedU128<frac::U99>;
/// [`FixedU128`](../struct.FixedU128.html) with 28 integer bits and 100 fractional bits.
pub type U28F100 = FixedU128<frac::U100>;
/// [`FixedU128`](../struct.FixedU128.html) with 27 integer bits and 101 fractional bits.
pub type U27F101 = FixedU128<frac::U101>;
/// [`FixedU128`](../struct.FixedU128.html) with 26 integer bits and 102 fractional bits.
pub type U26F102 = FixedU128<frac::U102>;
/// [`FixedU128`](../struct.FixedU128.html) with 25 integer bits and 103 fractional bits.
pub type U25F103 = FixedU128<frac::U103>;
/// [`FixedU128`](../struct.FixedU128.html) with 24 integer bits and 104 fractional bits.
pub type U24F104 = FixedU128<frac::U104>;
/// [`FixedU128`](../struct.FixedU128.html) with 23 integer bits and 105 fractional bits.
pub type U23F105 = FixedU128<frac::U105>;
/// [`FixedU128`](../struct.FixedU128.html) with 22 integer bits and 106 fractional bits.
pub type U22F106 = FixedU128<frac::U106>;
/// [`FixedU128`](../struct.FixedU128.html) with 21 integer bits and 107 fractional bits.
pub type U21F107 = FixedU128<frac::U107>;
/// [`FixedU128`](../struct.FixedU128.html) with 20 integer bits and 108 fractional bits.
pub type U20F108 = FixedU128<frac::U108>;
/// [`FixedU128`](../struct.FixedU128.html) with 19 integer bits and 109 fractional bits.
pub type U19F109 = FixedU128<frac::U109>;
/// [`FixedU128`](../struct.FixedU128.html) with 18 integer bits and 110 fractional bits.
pub type U18F110 = FixedU128<frac::U110>;
/// [`FixedU128`](../struct.FixedU128.html) with 17 integer bits and 111 fractional bits.
pub type U17F111 = FixedU128<frac::U111>;
/// [`FixedU128`](../struct.FixedU128.html) with 16 integer bits and 112 fractional bits.
pub type U16F112 = FixedU128<frac::U112>;
/// [`FixedU128`](../struct.FixedU128.html) with 15 integer bits and 113 fractional bits.
pub type U15F113 = FixedU128<frac::U113>;
/// [`FixedU128`](../struct.FixedU128.html) with 14 integer bits and 114 fractional bits.
pub type U14F114 = FixedU128<frac::U114>;
/// [`FixedU128`](../struct.FixedU128.html) with 13 integer bits and 115 fractional bits.
pub type U13F115 = FixedU128<frac::U115>;
/// [`FixedU128`](../struct.FixedU128.html) with 12 integer bits and 116 fractional bits.
pub type U12F116 = FixedU128<frac::U116>;
/// [`FixedU128`](../struct.FixedU128.html) with 11 integer bits and 117 fractional bits.
pub type U11F117 = FixedU128<frac::U117>;
/// [`FixedU128`](../struct.FixedU128.html) with 10 integer bits and 118 fractional bits.
pub type U10F118 = FixedU128<frac::U118>;
/// [`FixedU128`](../struct.FixedU128.html) with nine integer bits and 119 fractional bits.
pub type U9F119 = FixedU128<frac::U119>;
/// [`FixedU128`](../struct.FixedU128.html) with eight integer bits and 120 fractional bits.
pub type U8F120 = FixedU128<frac::U120>;
/// [`FixedU128`](../struct.FixedU128.html) with seven integer bits and 121 fractional bits.
pub type U7F121 = FixedU128<frac::U121>;
/// [`FixedU128`](../struct.FixedU128.html) with six integer bits and 122 fractional bits.
pub type U6F122 = FixedU128<frac::U122>;
/// [`FixedU128`](../struct.FixedU128.html) with five integer bits and 123 fractional bits.
pub type U5F123 = FixedU128<frac::U123>;
/// [`FixedU128`](../struct.FixedU128.html) with four integer bits and 124 fractional bits.
pub type U4F124 = FixedU128<frac::U124>;
/// [`FixedU128`](../struct.FixedU128.html) with three integer bits and 125 fractional bits.
pub type U3F125 = FixedU128<frac::U125>;
/// [`FixedU128`](../struct.FixedU128.html) with two integer bits and 126 fractional bits.
pub type U2F126 = FixedU128<frac::U126>;
/// [`FixedU128`](../struct.FixedU128.html) with one integer bit and 127 fractional bits.
pub type U1F127 = FixedU128<frac::U127>;
/// [`FixedU128`](../struct.FixedU128.html) with no integer bits and 128 fractional bits.
pub type U0F128 = FixedU128<frac::U128>;
