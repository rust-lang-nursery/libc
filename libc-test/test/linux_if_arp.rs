#![allow(bad_style, improper_ctypes, unused, deprecated)]

use libc::*;

#[cfg(any(target_os = "linux", target_os = "android"))]
include!(concat!(env!("OUT_DIR"), "/linux_if_arp.rs"));

#[cfg(not(any(target_os = "linux", target_os = "android")))]
fn main() {
    println!("PASSED 0 tests");
}
