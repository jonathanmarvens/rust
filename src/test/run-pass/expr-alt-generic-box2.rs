// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// xfail-fast
// -*- rust -*-

#[legacy_modes];

type compare<T> = fn@(T, T) -> bool;

fn test_generic<T: Copy>(expected: T, eq: compare<T>) {
    let actual: T = match true { true => { expected }, _ => die!(~"wat") };
    assert (eq(expected, actual));
}

fn test_vec() {
    fn compare_box(&&v1: @int, &&v2: @int) -> bool { return v1 == v2; }
    test_generic::<@int>(@1, compare_box);
}

pub fn main() { test_vec(); }
