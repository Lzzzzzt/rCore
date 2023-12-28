#![allow(unused)]

// legacy extensions: ignore fid
const SBI_SET_TIMER: usize = 0;
const SBI_CONSOLE_PUTCHAR: usize = 1;
const SBI_CONSOLE_GETCHAR: usize = 2;
const SBI_CLEAR_IPI: usize = 3;
const SBI_SEND_IPI: usize = 4;
const SBI_REMOTE_FENCE_I: usize = 5;
const SBI_REMOTE_SFENCE_VMA: usize = 6;
const SBI_REMOTE_SFENCE_VMA_ASID: usize = 7;

// system reset extension
const SRST_EXTENSION: usize = 0x53525354;
const SBI_SHUTDOWN: usize = 0;

#[inline(always)]
fn sbi_call(eid: usize, fid: usize, arg: [usize; 3]) -> usize {
    let mut ret;
    unsafe {
        core::arch::asm!(
            "ecall",
            inlateout("x10") arg[0] => ret,
            in("x11") arg[1],
            in("x12") arg[2],
            in("x16") fid,
            in("x17") eid,
        );
    }
    ret
}

pub fn console_put_char(c: usize) {
    sbi_call(SBI_CONSOLE_PUTCHAR, 0, [c, 0, 0]);
}

pub fn shutdown() -> ! {
    sbi_call(SRST_EXTENSION, SBI_SHUTDOWN, [0, 0, 0]);
    unreachable!()
}
