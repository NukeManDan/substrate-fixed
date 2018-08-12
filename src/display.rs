// Copyright © 2018 Trevor Spiteri

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

use core::cmp::Ordering;
use core::fmt::{
    Binary, Debug, Display, Formatter, LowerHex, Octal, Result as FmtResult, UpperHex,
};
use core::mem;
use core::str;
use frac::Unsigned;
use FixedHelper;

use {
    FixedI128, FixedI16, FixedI32, FixedI64, FixedI8, FixedU128, FixedU16, FixedU32, FixedU64,
    FixedU8,
};

trait Radix2 {
    fn digit_bits(&self) -> u32;
    fn prefix(&self) -> &'static str;
    fn encode_digits(&self, digits: &mut [u8]);
}

macro_rules! radix2 {
    ($Radix:ident($bits:expr, $prefix:expr), $($range:pat => $inc:expr),+) => {
        #[derive(Clone, Copy)]
        struct $Radix;
        impl Radix2 for $Radix {
            #[inline]
            fn digit_bits(&self) -> u32 {
                $bits
            }

            #[inline]
            fn prefix(&self) -> &'static str {
                $prefix
            }

            #[inline]
            fn encode_digits(&self, digits: &mut [u8]) {
                for digit in digits.iter_mut() {
                    *digit += match *digit {
                        $($range => $inc,)+
                        _ => continue,
                    };
                }
            }
        }
    };
}

radix2! { Bin(1, "0b"), 0..=1 => b'0' }
radix2! { Oct(3, "0o"), 0..=7 => b'0' }
radix2! { LowHex(4, "0x"), 0..=9 => b'0', 10..=15 => b'a' - 10 }
radix2! { UpHex(4, "0x"), 0..=9 => b'0', 10..=15 => b'A' - 10 }

trait FmtRadix2Helper {
    fn int_frac_bits() -> u32;
    fn is_zero(&self) -> bool;
    fn take_int_digit(&mut self, digit_bits: u32) -> u8;
    fn take_frac_digit(&mut self, digit_bits: u32) -> u8;
}

macro_rules! fmt_radix2_helper {
    ($($Inner:ty)*) => { $(
        impl FmtRadix2Helper for $Inner {
            #[inline]
            fn int_frac_bits() -> u32 {
                mem::size_of::<$Inner>() as u32 * 8
            }

            #[inline]
            fn is_zero(&self) -> bool {
                *self == 0
            }

            #[inline]
            fn take_int_digit(&mut self, digit_bits: u32) -> u8 {
                let mask = (1 << digit_bits) - 1;
                let ret = (*self & mask) as u8;
                *self >>= digit_bits;
                ret
            }

            #[inline]
            fn take_frac_digit(&mut self, digit_bits: u32) -> u8 {
                let int_frac_bits = <$Inner as FmtRadix2Helper>::int_frac_bits();
                let rem_bits = int_frac_bits - digit_bits;
                let mask = !0 << rem_bits;
                let ret = ((*self & mask) >> rem_bits) as u8;
                *self <<= digit_bits;
                ret
            }
        }
    )* };
}

fmt_radix2_helper! { u8 u16 u32 u64 u128 }

fn fmt_radix2_helper<F>(
    frac_bits: u32,
    (is_neg, mut int, mut frac): (bool, F, F),
    radix: &dyn Radix2,
    fmt: &mut Formatter,
) -> FmtResult
where
    F: FmtRadix2Helper,
{
    let int_bits = F::int_frac_bits() - frac_bits;
    let digit_bits: u32 = radix.digit_bits();
    // 128 binary digits, one radix point, one leading zero
    let mut buf: [u8; 130] = [0; 130];
    let max_int_digits = (int_bits + digit_bits - 1) / digit_bits;
    let frac_digits = (frac_bits + digit_bits - 1) / digit_bits;
    let (mut int_start, frac_start);
    if max_int_digits == 0 {
        buf[0] = b'0';
        buf[1] = b'.';
        int_start = 0;
        frac_start = 2;
    } else {
        int_start = max_int_digits;
        for r in buf[0..max_int_digits as usize].iter_mut().rev() {
            *r = int.take_int_digit(digit_bits);
            int_start -= 1;
            if int.is_zero() {
                break;
            }
        }
        buf[max_int_digits as usize] = b'.';
        frac_start = max_int_digits + 1;
    }
    let end;
    if frac_digits == 0 {
        end = frac_start - 1;
    } else {
        end = frac_start + frac_digits;
        for r in buf[frac_start as usize..end as usize].iter_mut() {
            *r = frac.take_frac_digit(digit_bits);
        }
    }
    let used_buf = &mut buf[int_start as usize..end as usize];
    radix.encode_digits(used_buf);
    let str_buf = str::from_utf8(used_buf).unwrap();
    fmt.pad_integral(!is_neg, radix.prefix(), str_buf)
}

#[inline]
fn fmt_radix2<Frac: Unsigned, F, Inner>(
    num: F,
    radix: &dyn Radix2,
    fmt: &mut Formatter,
) -> FmtResult
where
    F: FixedHelper<Frac, Inner = Inner>,
    Inner: FmtRadix2Helper,
{
    fmt_radix2_helper(Frac::to_u32(), num.parts(), radix, fmt)
}

macro_rules! impl_fmt {
    ($($Fixed:ident)*) => { $(
        impl<Frac: Unsigned> Display for $Fixed<Frac> {
            fn fmt(&self, f: &mut Formatter) -> FmtResult {
                fmt_dec(*self, f)
            }
        }
        impl<Frac: Unsigned>  Debug for $Fixed<Frac> {
            fn fmt(&self, f: &mut Formatter) -> FmtResult {
                fmt_dec(*self, f)
            }
        }
        impl<Frac: Unsigned>  Binary for $Fixed<Frac> {
            fn fmt(&self, f: &mut Formatter) -> FmtResult {
                fmt_radix2(*self, &Bin, f)
            }
        }
        impl<Frac: Unsigned>  Octal for $Fixed<Frac> {
            fn fmt(&self, f: &mut Formatter) -> FmtResult {
                fmt_radix2(*self, &Oct, f)
            }
        }
        impl<Frac: Unsigned>  LowerHex for $Fixed<Frac> {
            fn fmt(&self, f: &mut Formatter) -> FmtResult {
                fmt_radix2(*self, &LowHex, f)
            }
        }
        impl<Frac: Unsigned>  UpperHex for $Fixed<Frac> {
            fn fmt(&self, f: &mut Formatter) -> FmtResult {
                fmt_radix2(*self, &UpHex, f)
            }
        }
    )* };
}

impl_fmt!{ FixedU8 FixedU16 FixedU32 FixedU64 FixedU128 }
impl_fmt!{ FixedI8 FixedI16 FixedI32 FixedI64 FixedI128 }

fn dec_int_digits(int_bits: u32) -> u32 {
    assert!(int_bits < 299);
    if int_bits == 0 {
        return 0;
    }
    let i = if int_bits >= 196 {
        12
    } else if int_bits >= 103 {
        11
    } else {
        10
    };
    let digits = (int_bits * 3 + i) / 10;

    // check that digits is ceil(log10(2^int_bits - 1)), except when int_bits < 2
    #[cfg(feature = "std")]
    debug_assert!(
        int_bits < 2 || digits == (f64::from(int_bits).exp2() - 1.0).log10().ceil() as u32
    );

    digits
}

fn dec_frac_digits(frac_bits: u32) -> u32 {
    assert!(frac_bits < 300);
    let i = if frac_bits >= 197 {
        9
    } else if frac_bits >= 104 {
        8
    } else {
        7
    };
    let digits = (frac_bits * 3 + i) / 10;

    // check that error < delta, where
    // error = 0.5 * 10^-digits
    // delta = 2^-frac_bits
    #[cfg(feature = "std")]
    debug_assert!(0.5 * 10f64.powi(0 - digits as i32) < (-f64::from(frac_bits)).exp2());
    // check that error with one less digit >= delta
    #[cfg(feature = "std")]
    debug_assert!(0.5 * 10f64.powi(1 - digits as i32) >= (-f64::from(frac_bits)).exp2());

    digits
}

trait FmtDecHelper {
    fn int_frac_bits() -> u32;
    fn is_zero(&self) -> bool;
    fn cmp_half(&self) -> Ordering;
    fn take_int_digit(&mut self) -> u8;
    fn take_frac_digit(&mut self) -> u8;
}

macro_rules! fmt_dec_helper {
    ($($Inner:ty)*) => { $(
        impl FmtDecHelper for $Inner {
            #[inline]
            fn int_frac_bits() -> u32 {
                mem::size_of::<$Inner>() as u32 * 8
            }

            #[inline]
            fn is_zero(&self) -> bool {
                *self == 0
            }

            #[inline]
            fn cmp_half(&self) -> Ordering {
                self.cmp(&!(!0 >> 1))
            }

            #[inline]
            fn take_int_digit(&mut self) -> u8 {
                let ret = (*self % 10) as u8;
                *self /= 10;
                ret
            }

            #[inline]
            fn take_frac_digit(&mut self) -> u8 {
                let next = self.wrapping_mul(10);
                let ret = ((*self - next / 10) / (!0 / 10)) as u8;
                *self = next;
                ret
            }
        }
    )* };
}

fmt_dec_helper! { u8 u16 u32 u64 u128 }

fn fmt_dec_helper<F>(
    frac_bits: u32,
    (is_neg, mut int, mut frac): (bool, F, F),
    fmt: &mut Formatter,
) -> FmtResult
where
    F: FmtDecHelper,
{
    let int_bits = F::int_frac_bits() - frac_bits;
    // 40 int digits
    // + 128 frac digits
    // + 1 dec point,
    // + 1 leading zero or padding for carry due to rounding up,
    // = 170
    let mut buf: [u8; 170] = [0; 170];
    let max_int_digits = dec_int_digits(int_bits);
    let req_frac_digits = dec_frac_digits(frac_bits);
    // precision is limited to frac bits, which would always print
    // exact non-rounded number anyway
    let frac_digits = if let Some(prec) = fmt.precision().map(|x| x as u32) {
        if prec > frac_bits {
            frac_bits
        } else {
            prec
        }
    } else {
        req_frac_digits
    };
    let mut int_start;
    let frac_start;
    if max_int_digits == 0 {
        buf[0] = b'0';
        buf[1] = b'.';
        int_start = 0;
        frac_start = 2;
    } else {
        // pad by one in case rounding results in another digit
        int_start = max_int_digits + 1;
        buf[int_start as usize] = b'.';
        frac_start = int_start + 1;
        for r in buf[1..int_start as usize].iter_mut().rev() {
            *r = b'0' + int.take_int_digit();
            int_start -= 1;
            if int.is_zero() {
                break;
            }
        }
    }
    let end;
    if frac_digits == 0 {
        end = frac_start - 1;
    } else {
        end = frac_start + frac_digits;
        for r in buf[frac_start as usize..end as usize].iter_mut() {
            *r = b'0' + frac.take_frac_digit();
        }
        // check for rounding up
        let round_up = match frac.cmp_half() {
            Ordering::Less => false,
            Ordering::Greater => true,
            Ordering::Equal => {
                let last_digit = buf[(end - 1) as usize];
                debug_assert!(b'0' <= last_digit && last_digit <= b'9');
                // round up only if odd, so that we round to even
                (last_digit & 1) != 0
            }
        };
        if round_up {
            let mut done = false;
            for r in buf[int_start as usize..end as usize].iter_mut().rev() {
                if *r == b'9' {
                    *r = b'0';
                } else if *r != b'.' {
                    *r += 1;
                    done = true;
                    break;
                }
            }
            if !done {
                int_start -= 1;
                buf[int_start as usize] = b'1';
            }
        }
    }
    let buf = str::from_utf8(&buf[int_start as usize..end as usize]).unwrap();
    fmt.pad_integral(!is_neg, "", buf)
}

#[inline]
fn fmt_dec<Frac: Unsigned, F, Inner>(num: F, fmt: &mut Formatter) -> FmtResult
where
    F: FixedHelper<Frac, Inner = Inner>,
    Inner: FmtDecHelper,
{
    fmt_dec_helper(Frac::to_u32(), num.parts(), fmt)
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use *;

    #[test]
    fn hex() {
        use frac::U7 as Frac;
        let frac = Frac::to_u32();
        for i in 0..(1 << frac) {
            let p = 0x1234_5678_9abc_def0u64 ^ i as u64;
            let n = -0x1234_5678_9abc_def0i64 ^ i as i64;
            let f_p = FixedU64::<Frac>::from_bits(p);
            let f_n = FixedI64::<Frac>::from_bits(n);
            assert_eq!(
                format!("{:x}", f_p),
                format!("{:x}.{:02x}", p >> frac, (p & 0x7f) << 1)
            );
            assert_eq!(
                format!("{:x}", f_n),
                format!("-{:x}.{:02x}", n.abs() >> frac, (n.abs() & 0x7f) << 1)
            );
        }
    }

    #[test]
    fn dec() {
        use frac::U7 as Frac;
        let frac = Frac::to_u32();
        for i in 0..(1 << frac) {
            let bits = !0u32 ^ i;
            let flt = bits as f64 / (frac as f64).exp2();
            let fix = FixedU32::<Frac>::from_bits(bits);
            assert_eq!(format!("{}", fix), format!("{:.2}", flt));
        }
    }
}
