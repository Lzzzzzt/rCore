#![no_std]
#![feature(linkage)]
#![feature(panic_info_message)]

pub mod console;
mod syscall;

#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn _start() -> ! {
    clear_bss();
    exit(main());
}

#[linkage = "weak"]
#[no_mangle]
fn main() -> i32 {
    panic!("Cannot find main!");
}

fn clear_bss() {
    extern "C" {
        fn start_bss();
        fn end_bss();
    }
    (start_bss as usize..end_bss as usize).for_each(|addr| unsafe {
        (addr as *mut u8).write_volatile(0);
    });
}

#[panic_handler]
pub fn panic_handler(panic_info: &core::panic::PanicInfo) -> ! {
    let err = panic_info.message().unwrap();
    if let Some(location) = panic_info.location() {
        println!(
            "Panicked at {}:{}, {}",
            location.file(),
            location.line(),
            err
        );
    } else {
        println!("Panicked: {}", err);
    }
    exit(-1);
}

use core::time::Duration;
use syscall::*;

pub fn write(fd: usize, buf: &[u8]) -> isize {
    sys_write(fd, buf)
}
pub fn exit(exit_code: i32) -> ! {
    sys_exit(exit_code);
    unreachable!()
}
pub fn r#yield() -> isize {
    sys_yield()
}

pub fn get_time() -> Duration {
    Duration::from_micros(sys_get_time() as u64)
}

pub fn sleep(ms: u64) {
    let wait_for = sys_get_time() + ms as isize * 1000;

    while sys_get_time() < wait_for {
        sys_yield();
    }
}
