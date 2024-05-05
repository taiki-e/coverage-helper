// SPDX-License-Identifier: Apache-2.0 OR MIT

#![allow(unexpected_cfgs)] // for cfg(coverage_nightly)
#![cfg_attr(all(coverage_nightly, test), feature(coverage_attribute))]
#![allow(dead_code)]

#[coverage_helper::test]
fn a() {
    if true {
        b();
    } else {
        b();
    }
}

fn b() {
    println!();
}
