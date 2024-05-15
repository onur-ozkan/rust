//! Replacement of `bootstrap.py`
//!
//! TODOs
//! - download beta compiler (What will happen when cargo and rustc is set in the config.toml?)
//! - Migrate all the python logic to a new bootstrap binary (something like bootstrap-init) that starts bootstrapping at the end of the process, similar to bootstrap.py.
//! - Run `cargo run` on the migrated program.
//!
//! - Do not download already downloaded files
//!
//! Move certain options from config.toml to environment variables,
//! so we can read them here easily as we can't parse config.toml
//! in bash scripts.

#![allow(dead_code)]

use std::path::PathBuf;

struct Init {
    build_dir: PathBuf,
    clean: bool,
    stage0_sysroot: PathBuf,
}

fn main() {
    // 'PATH': '$build_dir/x86_64-unknown-linux-gnu/stage0/bin:$REST'
    // 'CARGO_TARGET_DIR': '$build_dir/bootstrap'
    // 'RUSTC': '$build_dir/x86_64-unknown-linux-gnu/stage0/bin/rustc'
    // 'LD_LIBRARY_PATH': ''
    // 'DYLD_LIBRARY_PATH': ''
    // 'LIBRARY_PATH': ''
    // 'LIBPATH': ''
    // 'RUSTC_BOOTSTRAP': '1'
    // 'RUSTFLAGS': '-Zallow-features= -Wrust_2018_idioms -Wunused_lifetimes -Dwarnings'
    for (key, value) in std::env::vars() {
        println!("{key}: {value}");
    }
}
