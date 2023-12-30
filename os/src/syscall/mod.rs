use self::{fs::sys_write, process::*};
use log::error;

pub mod fs;
pub mod process;

const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_YIELD: usize = 124;
const SYSCALL_GET_TIME: usize = 169;

pub fn syscall(id: usize, args: [usize; 3]) -> isize {
    match id {
        SYSCALL_EXIT => sys_exit(args[0] as i32),
        SYSCALL_WRITE => sys_write(args[0], args[1] as *mut u8, args[2]),
        SYSCALL_YIELD => sys_yield(),
        SYSCALL_GET_TIME => sys_get_time(),
        _ => {
            error!("Unimplement Syscall: {}", id);
            sys_exit(-1)
        }
    }
}
