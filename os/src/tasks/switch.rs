use log::info;

use crate::timer::Time;

use super::context::TaskContext;
use core::{arch::global_asm, ops::AddAssign};

global_asm!(include_str!("switch.S"));

static mut SWITCH_START: Time = Time::zero();
static mut SWITCH_COST: Time = Time::zero();

extern "C" {
    pub fn __switch(cur_task_ctx_p: *mut TaskContext, nxt_task_ctx_p: *const TaskContext);
}

pub fn _switch(cur_task_ctx_p: *mut TaskContext, nxt_task_ctx_p: *const TaskContext) {
    unsafe {
        SWITCH_START = Time::now();
        __switch(cur_task_ctx_p, nxt_task_ctx_p);
        SWITCH_COST.add_assign(SWITCH_START.elapsed());
    }
}

pub fn print_switch_cost() {
    info!("Switch App Cost: {:.4}ms", unsafe {
        SWITCH_COST.as_secs_f32() * 1000.
    })
}
