// SPDX-License-Identifier: Apache-2.0 OR MIT

#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
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
