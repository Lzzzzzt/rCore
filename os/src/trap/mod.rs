use self::context::TrapContext;
use crate::syscall::process::sys_exit;
use crate::syscall::syscall;
use crate::tasks::{self, suspend_current_then_run_next};
use crate::timer::set_next_trigger;
use log::{debug, error, warn};
use riscv::register::scause::{self, Exception, Interrupt, Trap};
use riscv::register::{sstatus, stval, stvec};

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

pub fn enable_timer_interrupt() {
    unsafe { riscv::register::sie::set_stimer() }
}

/// external function __all_traps will call this function automatically,
/// after executing this function, cpu will execute __trap_return
#[no_mangle]
pub fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    match sstatus::read().spp() {
        sstatus::SPP::Supervisor => kernel_trap_handler(cx),
        sstatus::SPP::User => user_trap_handler(cx),
    }
}

static mut KT_MARKER: bool = false;

fn kernel_trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    let scause = scause::read();
    let stval = stval::read();

    match scause.cause() {
        Trap::Interrupt(Interrupt::SupervisorTimer) => unsafe {
            if !KT_MARKER {
                KT_MARKER = true;
                debug!("SupervisorTimer Interrupt Has Been Trigger!");
            }
        },
        _ => {
            error!("{:?} not supported, stval = {:#x}", scause.cause(), stval);
            sys_exit(-1);
        }
    }

    cx
}

fn user_trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    tasks::calc_user_time();

    let scause = scause::read();
    let stval = stval::read();

    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            cx.sepc += 4;
            cx.x[10] = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize;
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            warn!("Illegal Instruction Found, Kill It!");
            sys_exit(-1);
        }
        Trap::Exception(Exception::StorePageFault | Exception::StoreFault) => {
            warn!("Store Page Fault Found, Kill It!");
            sys_exit(-1);
        }
        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            set_next_trigger();
            suspend_current_then_run_next();
        }
        _ => {
            error!("{:?} not supported, stval = {:#x}", scause.cause(), stval);
            sys_exit(-1);
        }
    }

    tasks::calc_kernel_time();
    cx
}
