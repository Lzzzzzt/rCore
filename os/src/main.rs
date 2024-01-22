#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

mod config;
mod console;
mod entry;
mod lang;
mod loader;
mod log;
mod memory_map;
mod sbi;
mod sync;
mod syscall;
mod tasks;
mod timer;
mod trap;
mod utils;

fn main() {
    tasks::run_first();
}
