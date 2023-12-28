use log::info;

use crate::batch::run_next_app;

pub fn sys_exit(state: i32) -> ! {
    info!("[KERNEL] Exited with {}", state);
    run_next_app()
}
