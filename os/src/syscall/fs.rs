use log::warn;

use crate::{batch::check_address_range, print};

const FD_STDOUT: usize = 1;

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    if !check_address_range(buf, len) {
        let start = buf as usize;
        let end = buf as usize + len;

        warn!("[KERNEL] Accessing Wrong Addr: [{:#x}, {:#x})", start, end);
        return -1;
    }

    match fd {
        FD_STDOUT => {
            let output_bytes = unsafe { core::slice::from_raw_parts(buf, len) };
            let output_str = core::str::from_utf8(output_bytes).unwrap();
            print!("{}", output_str);
            len as isize
        }
        _ => {
            warn!("unimplement write target");
            -1
        }
    }
}
