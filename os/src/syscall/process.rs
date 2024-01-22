use crate::{tasks, timer};
use log::info;

pub fn sys_exit(state: i32) -> ! {
    info!("Exited with {}", state);
    tasks::exit_current_then_run_next();
    unreachable!()
}

pub fn sys_yield() -> isize {
    tasks::suspend_current_then_run_next();
    0
}

pub fn sys_get_time() -> isize {
    timer::Time::current_up_time().as_micros() as isize
}
