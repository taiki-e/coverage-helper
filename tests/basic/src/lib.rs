#![cfg_attr(coverage_nightly, feature(no_coverage))]
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
