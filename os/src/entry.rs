use crate::log::init_logger;
use crate::sbi::shutdown;
use log::info;

core::arch::global_asm!(include_str!("entry.asm"));
core::arch::global_asm!(include_str!("link_app.S"));

#[no_mangle]
pub fn rust_main() -> ! {
    init();
    crate::main();
    shutdown(false);
}

fn init() {
    clear_bss();
    init_logger();

    extern "C" {
        fn stext();
        fn etext();

        fn srodata();
        fn erodata();

        fn sdata();
        fn edata();

        fn sbss();
        fn ebss();
    }

    info!(".TEXT   [{:#x}, {:#x})", stext as usize, etext as usize);
    info!(".RODATA [{:#x}, {:#x})", srodata as usize, erodata as usize);
    info!(".DATA   [{:#x}, {:#x})", sdata as usize, edata as usize);
    info!(".BSS    [{:#x}, {:#x})", sbss as usize, ebss as usize);
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }

    unsafe { (sbss as usize..ebss as usize).for_each(|m| (m as *mut u8).write_volatile(0)) }
}


