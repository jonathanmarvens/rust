// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// NB: transitionary, de-mode-ing.
#[forbid(deprecated_mode)];
#[forbid(deprecated_pattern)];

use T = self::inst::T;
use T_SIGNED = self::inst::T_SIGNED;

use char;
use cmp::{Eq, Ord};
use cmp;
use to_str::ToStr;
use from_str::FromStr;
use num::{ToStrRadix, FromStrRadix};
use num;
use option::{None, Option, Some};
use prelude::*;
use str;
use uint;
use vec;
use u8;
use u16;
use u32;

pub const bits : uint = inst::bits;
pub const bytes : uint = (inst::bits / 8);

pub const min_value: T = 0 as T;
pub const max_value: T = 0 as T - 1 as T;

#[inline(always)]
pub pure fn min(x: T, y: T) -> T { if x < y { x } else { y } }
#[inline(always)]
pub pure fn max(x: T, y: T) -> T { if x > y { x } else { y } }

#[inline(always)]
pub pure fn add(x: T, y: T) -> T { x + y }
#[inline(always)]
pub pure fn sub(x: T, y: T) -> T { x - y }
#[inline(always)]
pub pure fn mul(x: T, y: T) -> T { x * y }
#[inline(always)]
pub pure fn div(x: T, y: T) -> T { x / y }
#[inline(always)]
pub pure fn rem(x: T, y: T) -> T { x % y }

#[inline(always)]
pub pure fn lt(x: T, y: T) -> bool { x < y }
#[inline(always)]
pub pure fn le(x: T, y: T) -> bool { x <= y }
#[inline(always)]
pub pure fn eq(x: T, y: T) -> bool { x == y }
#[inline(always)]
pub pure fn ne(x: T, y: T) -> bool { x != y }
#[inline(always)]
pub pure fn ge(x: T, y: T) -> bool { x >= y }
#[inline(always)]
pub pure fn gt(x: T, y: T) -> bool { x > y }

#[inline(always)]
pub pure fn is_positive(x: T) -> bool { x > 0 as T }
#[inline(always)]
pub pure fn is_negative(x: T) -> bool { x < 0 as T }
#[inline(always)]
pub pure fn is_nonpositive(x: T) -> bool { x <= 0 as T }
#[inline(always)]
pub pure fn is_nonnegative(x: T) -> bool { x >= 0 as T }

#[inline(always)]
/**
 * Iterate over the range [`start`,`start`+`step`..`stop`)
 *
 */
pub pure fn range_step(start: T, stop: T, step: T_SIGNED, it: fn(T) -> bool) {
    let mut i = start;
    if step == 0 {
        die!(~"range_step called with step == 0");
    }
    if step >= 0 {
        while i < stop {
            if !it(i) { break }
            i += step as T;
        }
    }
    else {
        while i > stop {
            if !it(i) { break }
            i -= -step as T;
        }
    }
}

#[inline(always)]
/// Iterate over the range [`lo`..`hi`)
pub pure fn range(lo: T, hi: T, it: fn(T) -> bool) {
    range_step(lo, hi, 1 as T_SIGNED, it);
}

#[inline(always)]
/// Iterate over the range [`hi`..`lo`)
pub pure fn range_rev(hi: T, lo: T, it: fn(T) -> bool) {
    range_step(hi, lo, -1 as T_SIGNED, it);
}

/// Computes the bitwise complement
#[inline(always)]
pub pure fn compl(i: T) -> T {
    max_value ^ i
}

#[cfg(notest)]
impl T : Ord {
    #[inline(always)]
    pure fn lt(&self, other: &T) -> bool { (*self) < (*other) }
    #[inline(always)]
    pure fn le(&self, other: &T) -> bool { (*self) <= (*other) }
    #[inline(always)]
    pure fn ge(&self, other: &T) -> bool { (*self) >= (*other) }
    #[inline(always)]
    pure fn gt(&self, other: &T) -> bool { (*self) > (*other) }
}

#[cfg(notest)]
impl T : Eq {
    #[inline(always)]
    pure fn eq(&self, other: &T) -> bool { return (*self) == (*other); }
    #[inline(always)]
    pure fn ne(&self, other: &T) -> bool { return (*self) != (*other); }
}

impl T: num::Num {
    #[inline(always)]
    pure fn add(&self, other: &T)    -> T { return *self + *other; }
    #[inline(always)]
    pure fn sub(&self, other: &T)    -> T { return *self - *other; }
    #[inline(always)]
    pure fn mul(&self, other: &T)    -> T { return *self * *other; }
    #[inline(always)]
    pure fn div(&self, other: &T)    -> T { return *self / *other; }
    #[inline(always)]
    pure fn modulo(&self, other: &T) -> T { return *self % *other; }
    #[inline(always)]
    pure fn neg(&self)              -> T { return -*self;        }

    #[inline(always)]
    pure fn to_int(&self)         -> int { return *self as int; }
    #[inline(always)]
    static pure fn from_int(n: int) -> T   { return n as T;      }
}

impl T: num::Zero {
    #[inline(always)]
    static pure fn zero() -> T { 0 }
}

impl T: num::One {
    #[inline(always)]
    static pure fn one() -> T { 1 }
}

impl T: num::Round {
    #[inline(always)]
    pure fn round(&self, _: num::RoundMode) -> T { *self }

    #[inline(always)]
    pure fn floor(&self) -> T { *self }
    #[inline(always)]
    pure fn ceil(&self) -> T { *self }
    #[inline(always)]
    pure fn fract(&self) -> T { 0 }
}

// String conversion functions and impl str -> num

/// Parse a string as a number in base 10.
#[inline(always)]
pub pure fn from_str(s: &str) -> Option<T> {
    num::from_str_common(s, 10u, false, false, false,
                         num::ExpNone, false)
}

/// Parse a string as a number in the given base.
#[inline(always)]
pub pure fn from_str_radix(s: &str, radix: uint) -> Option<T> {
    num::from_str_common(s, radix, false, false, false,
                         num::ExpNone, false)
}

/// Parse a byte slice as a number in the given base.
#[inline(always)]
pub pure fn parse_bytes(buf: &[u8], radix: uint) -> Option<T> {
    num::from_str_bytes_common(buf, radix, false, false, false,
                               num::ExpNone, false)
}

impl T : FromStr {
    #[inline(always)]
    static pure fn from_str(s: &str) -> Option<T> {
        from_str(s)
    }
}

impl T : FromStrRadix {
    #[inline(always)]
    static pure fn from_str_radix(&self, s: &str, radix: uint) -> Option<T> {
        from_str_radix(s, radix)
    }
}

// String conversion functions and impl num -> str

/// Convert to a string as a byte slice in a given base.
#[inline(always)]
pub pure fn to_str_bytes<U>(n: T, radix: uint, f: fn(v: &[u8]) -> U) -> U {
    let (buf, _) = num::to_str_bytes_common(&n, radix, false, false,
                                            num::SignNeg, num::DigAll);
    f(buf)
}

/// Convert to a string in base 10.
#[inline(always)]
pub pure fn to_str(num: T) -> ~str {
    let (buf, _) = num::to_str_common(&num, 10u, false, false,
                                      num::SignNeg, num::DigAll);
    buf
}

/// Convert to a string in a given base.
#[inline(always)]
pub pure fn to_str_radix(num: T, radix: uint) -> ~str {
    let (buf, _) = num::to_str_common(&num, radix, false, false,
                                      num::SignNeg, num::DigAll);
    buf
}

/// Convert to a string.
/// *Deprecated*, use to_str() instead.
#[inline(always)]
pub pure fn str(i: T) -> ~str { to_str(i) }

impl T : ToStr {
    #[inline(always)]
    pure fn to_str(&self) -> ~str {
        to_str(*self)
    }
}

impl T : ToStrRadix {
    #[inline(always)]
    pure fn to_str_radix(&self, radix: uint) -> ~str {
        to_str_radix(*self, radix)
    }
}

#[test]
pub fn test_to_str() {
    assert to_str_radix(0 as T, 10u) == ~"0";
    assert to_str_radix(1 as T, 10u) == ~"1";
    assert to_str_radix(2 as T, 10u) == ~"2";
    assert to_str_radix(11 as T, 10u) == ~"11";
    assert to_str_radix(11 as T, 16u) == ~"b";
    assert to_str_radix(255 as T, 16u) == ~"ff";
    assert to_str_radix(0xff as T, 10u) == ~"255";
}

#[test]
pub fn test_from_str() {
    assert from_str(~"0") == Some(0u as T);
    assert from_str(~"3") == Some(3u as T);
    assert from_str(~"10") == Some(10u as T);
    assert u32::from_str(~"123456789") == Some(123456789 as u32);
    assert from_str(~"00100") == Some(100u as T);

    assert from_str(~"").is_none();
    assert from_str(~" ").is_none();
    assert from_str(~"x").is_none();
}

#[test]
pub fn test_parse_bytes() {
    use str::to_bytes;
    assert parse_bytes(to_bytes(~"123"), 10u) == Some(123u as T);
    assert parse_bytes(to_bytes(~"1001"), 2u) == Some(9u as T);
    assert parse_bytes(to_bytes(~"123"), 8u) == Some(83u as T);
    assert u16::parse_bytes(to_bytes(~"123"), 16u) == Some(291u as u16);
    assert u16::parse_bytes(to_bytes(~"ffff"), 16u) == Some(65535u as u16);
    assert parse_bytes(to_bytes(~"z"), 36u) == Some(35u as T);

    assert parse_bytes(to_bytes(~"Z"), 10u).is_none();
    assert parse_bytes(to_bytes(~"_"), 2u).is_none();
}

#[test]
fn test_uint_to_str_overflow() {
    let mut u8_val: u8 = 255_u8;
    assert (u8::to_str(u8_val) == ~"255");

    u8_val += 1 as u8;
    assert (u8::to_str(u8_val) == ~"0");

    let mut u16_val: u16 = 65_535_u16;
    assert (u16::to_str(u16_val) == ~"65535");

    u16_val += 1 as u16;
    assert (u16::to_str(u16_val) == ~"0");

    let mut u32_val: u32 = 4_294_967_295_u32;
    assert (u32::to_str(u32_val) == ~"4294967295");

    u32_val += 1 as u32;
    assert (u32::to_str(u32_val) == ~"0");

    let mut u64_val: u64 = 18_446_744_073_709_551_615_u64;
    assert (u64::to_str(u64_val) == ~"18446744073709551615");

    u64_val += 1 as u64;
    assert (u64::to_str(u64_val) == ~"0");
}

#[test]
fn test_uint_from_str_overflow() {
    let mut u8_val: u8 = 255_u8;
    assert (u8::from_str(~"255") == Some(u8_val));
    assert (u8::from_str(~"256").is_none());

    u8_val += 1 as u8;
    assert (u8::from_str(~"0") == Some(u8_val));
    assert (u8::from_str(~"-1").is_none());

    let mut u16_val: u16 = 65_535_u16;
    assert (u16::from_str(~"65535") == Some(u16_val));
    assert (u16::from_str(~"65536").is_none());

    u16_val += 1 as u16;
    assert (u16::from_str(~"0") == Some(u16_val));
    assert (u16::from_str(~"-1").is_none());

    let mut u32_val: u32 = 4_294_967_295_u32;
    assert (u32::from_str(~"4294967295") == Some(u32_val));
    assert (u32::from_str(~"4294967296").is_none());

    u32_val += 1 as u32;
    assert (u32::from_str(~"0") == Some(u32_val));
    assert (u32::from_str(~"-1").is_none());

    let mut u64_val: u64 = 18_446_744_073_709_551_615_u64;
    assert (u64::from_str(~"18446744073709551615") == Some(u64_val));
    assert (u64::from_str(~"18446744073709551616").is_none());

    u64_val += 1 as u64;
    assert (u64::from_str(~"0") == Some(u64_val));
    assert (u64::from_str(~"-1").is_none());
}

#[test]
#[should_fail]
#[ignore(cfg(windows))]
pub fn to_str_radix1() {
    uint::to_str_radix(100u, 1u);
}

#[test]
#[should_fail]
#[ignore(cfg(windows))]
pub fn to_str_radix37() {
    uint::to_str_radix(100u, 37u);
}

use io;
#[test]
pub fn test_ranges() {
    let mut l = ~[];

    for range(0,3) |i| {
        l.push(i);
    }
    for range_rev(13,10) |i| {
        l.push(i);
    }
    for range_step(20,26,2) |i| {
        l.push(i);
    }
    for range_step(36,30,-2) |i| {
        l.push(i);
    }

    assert l == ~[0,1,2,
                  13,12,11,
                  20,22,24,
                  36,34,32];

    // None of the `fail`s should execute.
    for range(0,0) |_i| {
        die!(~"unreachable");
    }
    for range_rev(0,0) |_i| {
        die!(~"unreachable");
    }
    for range_step(10,0,1) |_i| {
        die!(~"unreachable");
    }
    for range_step(0,1,-10) |_i| {
        die!(~"unreachable");
    }
}

#[test]
#[should_fail]
#[ignore(cfg(windows))]
fn test_range_step_zero_step_up() {
    for range_step(0,10,0) |_i| {}
}
#[test]
#[should_fail]
#[ignore(cfg(windows))]
fn test_range_step_zero_step_down() {
    for range_step(0,-10,0) |_i| {}
}
