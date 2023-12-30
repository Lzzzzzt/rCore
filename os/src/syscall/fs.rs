use crate::print;
use log::warn;

const FD_STDOUT: usize = 1;

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
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
