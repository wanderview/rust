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
// aux-build:static-methods-crate.rs

extern mod static_methods_crate;
use static_methods_crate::read;
use readMaybeRenamed = static_methods_crate::read::readMaybe;

pub fn main() {
    let result: int = read(~"5");
    fail_unless!(result == 5);
    fail_unless!(readMaybeRenamed(~"false") == Some(false));
    fail_unless!(readMaybeRenamed(~"foo") == None::<bool>);
}
