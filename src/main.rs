#![no_std]
#![no_main]

extern crate alloc;

#[cfg_attr(target_os = "linux", no_main)]
use noli::prelude::*;
entry_point!(main);

fn main() -> u64 {
    println!("Hello, world!");

    0
}
