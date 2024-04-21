// SPDX-License-Identifier: Apache-2.0 OR MIT

// The rustc-cfg emitted by the build script are *not* public API.

use std::{
    env,
    io::Write,
    iter,
    process::{Command, Stdio},
};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    if probe_feature("coverage_attribute").unwrap_or(false) {
        println!("cargo:rustc-cfg=coverage_helper_has_coverage_attribute");
    }
}

// Refs:
// - https://github.com/cuviper/autocfg/blob/e75324cfa5fa0bec4f06f9609ab169bb7747f984/src/lib.rs#L205
// - https://github.com/dtolnay/anyhow/blob/7ecec7a3799b94c3b58a6b4cd050ff60cba99129/build.rs#L70
fn probe_feature(feature_name: &str) -> Option<bool> {
    // https://github.com/dtolnay/thiserror/pull/248
    if env::var_os("RUSTC_STAGE").is_some() {
        return None;
    }

    let out_dir = env::var_os("OUT_DIR")?;
    let rustc = env::var_os("RUSTC")?;
    let (rustc_wrapper, rustc_workspace_wrapper) =
        if env::var_os("CARGO_ENCODED_RUSTFLAGS").is_some() {
            (
                env::var_os("RUSTC_WRAPPER").filter(|v| !v.is_empty()),
                env::var_os("RUSTC_WORKSPACE_WRAPPER").filter(|v| !v.is_empty()),
            )
        } else {
            // Cargo sets environment variables for wrappers correctly only since https://github.com/rust-lang/cargo/pull/9601.
            (None, None)
        };
    let mut rustc =
        rustc_wrapper.into_iter().chain(rustc_workspace_wrapper).chain(iter::once(rustc));
    let mut cmd = Command::new(rustc.next().unwrap());
    cmd.args(rustc);
    cmd.stderr(Stdio::null())
        .arg("--edition=2018")
        .arg("--crate-name")
        .arg(format!("coverage_helper_build_{}", feature_name))
        .arg("--crate-type=lib")
        .arg("--emit=metadata")
        .arg("--cap-lints=allow")
        .arg("--out-dir")
        .arg(out_dir);

    cmd.arg("-").stdin(Stdio::piped());
    let mut child = cmd.spawn().ok()?;
    let mut stdin = child.stdin.take().expect("rustc stdin");

    // There is no need to respect TARGET since #[coverage] is platform-independent,
    // just check for host. And host always has std.
    stdin.write_all(br"#![feature(").ok()?;
    stdin.write_all(feature_name.as_bytes()).ok()?;
    stdin.write_all(br")]").ok()?;

    drop(stdin);

    let status = child.wait().ok()?;
    Some(status.success())
}
