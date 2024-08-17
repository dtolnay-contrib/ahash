#![deny(warnings)]

use std::env;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    if let Some(true) = version_check::is_min_version("1.71.0") {
        println!("cargo:rustc-cfg=build_hasher_hash_one");
    }
    let arch = env::var("CARGO_CFG_TARGET_ARCH").expect("CARGO_CFG_TARGET_ARCH was not set");
    if arch.eq_ignore_ascii_case("x86_64")
        || arch.eq_ignore_ascii_case("aarch64")
        || arch.eq_ignore_ascii_case("mips64")
        || arch.eq_ignore_ascii_case("powerpc64")
        || arch.eq_ignore_ascii_case("riscv64gc")
        || arch.eq_ignore_ascii_case("s390x")
    {
        println!("cargo:rustc-cfg=feature=\"folded_multiply\"");
    }
}
