// Compiler:
//
// Run-time:
//   stdout: 2
//     7 8
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
    intrinsics,
    Sized,
    Copy,
    Receiver,
    Freeze,
    Add,
    panic
};


/*
 * Code
 */

fn inc_ref(num: &mut isize) -> isize {
    *num = *num + 5;
    *num + 1
}

fn inc(num: isize) -> isize {
    num + 1
}


#[start]
fn main(mut argc: isize, _argv: *const *const u8) -> isize {
    argc = inc(argc);
    unsafe {
        libc::printf(b"%ld\n\0" as *const u8 as *const i8, argc);
    }

    let b = inc_ref(&mut argc);
    unsafe {
        libc::printf(b"%ld %ld\n\0" as *const u8 as *const i8, argc, b);
    }

    argc = 10;
    unsafe {
        libc::printf(b"%ld\n\0" as *const u8 as *const i8, argc);
    }
    0
}
