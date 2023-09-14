// #![cfg_attr(coverage_nightly, feature(coverage_attribute))] // TODO: https://github.com/rust-lang/rust/pull/114656 not yet released
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
