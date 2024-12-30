// Compiler:
//
// Run-time:
//   stdout: 41
//     39
//     10

#![allow(internal_features, unused_attributes)]
#![feature(no_core, start)]

#![no_std]
#![no_core]

/*
 * Core
 */

extern crate mini_core;
use mini_core::{
    libc,
    Add,
    Sub,
    Mul,
    Deref,
    Sized,
    Copy,
    Receiver,
    Freeze
};

/*
 * Code
 */

#[start]
fn main(mut argc: isize, _argv: *const *const u8) -> isize {
    unsafe {
        libc::printf(b"%ld\n\0" as *const u8 as *const i8, 40 + argc);
        libc::printf(b"%ld\n\0" as *const u8 as *const i8, 40 - argc);
        libc::printf(b"%ld\n\0" as *const u8 as *const i8, 10 * argc);
    }
    0
}
