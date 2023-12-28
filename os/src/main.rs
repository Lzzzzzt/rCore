#![no_std]
#![no_main]
#![feature(panic_info_message)]

mod batch;
mod console;
mod entry;
mod lang;
mod log;
mod sbi;
mod sync;
mod syscall;
mod trap;

pub fn main() {
    trap::init();
    batch::init();
    batch::run_next_app();
}
