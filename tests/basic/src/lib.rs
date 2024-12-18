// SPDX-License-Identifier: Apache-2.0 OR MIT

#![allow(dead_code, deprecated)]

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
