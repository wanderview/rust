// -*- rust -*-
// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


#[cfg(target_arch = "x86")]
fn target() {
    fail_unless!((-1000 as uint >> 3u == 536870787u));
}

#[cfg(target_arch = "x86_64")]
fn target() {
    fail_unless!((-1000 as uint >> 3u == 2305843009213693827u));
}

fn general() {
    let mut a: int = 1;
    let mut b: int = 2;
    a ^= b;
    b ^= a;
    a = a ^ b;
    log(debug, a);
    log(debug, b);
    fail_unless!((b == 1));
    fail_unless!((a == 2));
    fail_unless!((!0xf0 & 0xff == 0xf));
    fail_unless!((0xf0 | 0xf == 0xff));
    fail_unless!((0xf << 4 == 0xf0));
    fail_unless!((0xf0 >> 4 == 0xf));
    fail_unless!((-16 >> 2 == -4));
    fail_unless!((0b1010_1010 | 0b0101_0101 == 0xff));
}

pub fn main() {
    general();
    target();
}
