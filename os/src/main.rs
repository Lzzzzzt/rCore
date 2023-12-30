#![no_std]
#![no_main]
#![feature(panic_info_message)]

mod config;
mod console;
mod entry;
mod lang;
mod loader;
mod log;
mod sbi;
mod sync;
mod syscall;
mod tasks;
mod timer;
mod trap;
mod utils;

fn main() {
    trap::init();
    loader::load_apps();
    trap::enable_timer_interrupt();
    utils::enable_float_ins();
    timer::set_next_trigger();
    tasks::run_first();
}
