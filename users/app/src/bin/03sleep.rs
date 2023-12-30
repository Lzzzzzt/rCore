#![no_std]
#![no_main]

use lib::sleep;

#[macro_use]
extern crate lib;

#[no_mangle]
fn main() -> i32 {
    sleep(3000);
    println!("Test sleep OK!");
    0
}
