use self::context::TrapContext;
use crate::{batch::run_next_app, syscall::syscall};
use log::warn;
use riscv::register::{
    scause::{self, Exception, Trap},
    stval, stvec,
};

pub mod context;

core::arch::global_asm!(include_str!("trap.S"));

pub fn init() {
    extern "C" {
        fn __all_traps();
    }

    unsafe {
        stvec::write(__all_traps as usize, stvec::TrapMode::Direct);
    }
}

#[no_mangle]
pub fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    let scause = scause::read();
    let stval = stval::read();

    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            cx.sepc += 4;
            cx.x[10] = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize;
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            warn!(" IllegalInstruction Found, Kill it!");
            run_next_app();
        }
        Trap::Exception(Exception::StorePageFault | Exception::StoreFault) => {
            warn!(" Store Page Fault Found, Kill it!");
            run_next_app();
        }
        _ => panic!("{:?} not supported, stval = {:#x}", scause.cause(), stval),
    }

    cx
}
