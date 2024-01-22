use self::switch::_switch;
use self::task::TaskControlBlock;

use crate::config::*;
use crate::loader::init_app_context;
use crate::sbi::shutdown;
use crate::sync::UPSafeCell;
use crate::tasks::context::TaskContext;
use crate::tasks::switch::print_switch_cost;
use crate::tasks::task::TaskStatus;
use crate::timer::Time;
use crate::utils::get_app_num;

use core::ops::AddAssign;
use core::time::Duration;

use lazy_static::lazy_static;
use log::{debug, info};

mod context;
mod switch;
mod task;

lazy_static! {
    pub static ref TASK_MANAGER: TaskManager = {
        let app_num = get_app_num();

        let mut tasks = [TaskControlBlock::uninit(); MAX_APP_NUM];

        // Load Each App
        (0..app_num).for_each(|i| {
            tasks[i].context = TaskContext::goto_ret_to_user(init_app_context(i));
            tasks[i].status = TaskStatus::Ready;
        });

        TaskManager {
            app_num,
            inner: unsafe { UPSafeCell::new(TaskManagerInner { tasks, current: 0, time: Time::current_up_time() }) },
        }
    };
}

pub struct TaskManager {
    app_num: usize,
    inner: UPSafeCell<TaskManagerInner>,
}

impl TaskManager {
    fn print_current_tcb(&self) {
        let inner = self.inner.exclusive_access();
        let current = inner.current;
        let current_tcb = inner.tasks[current];
        info!(
            "App {}(u: {}ms, k: {}ms)",
            current,
            current_tcb.user_time.as_millis(),
            current_tcb.kernel_time.as_millis()
        )
    }

    fn calc_kernel_time(&self) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current;
        let executing_time = inner.refresh_current_time();
        inner.tasks[current].kernel_time.add_assign(executing_time);
    }

    fn calc_user_time(&self) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current;
        let executing_time = inner.refresh_current_time();
        inner.tasks[current].user_time.add_assign(executing_time);
    }

    fn mark_current_status(&self, status: TaskStatus) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current;
        inner.tasks[current].status = status;
        let executing_time = inner.refresh_current_time();
        inner.tasks[current].kernel_time.add_assign(executing_time);
    }

    fn find_next(&self) -> Option<usize> {
        let inner = self.inner.exclusive_access();
        let current = inner.current;

        ((current + 1)..(current + self.app_num + 1))
            .map(|id| id % self.app_num)
            .find(|&id| inner.tasks[id].is_ready())
    }

    fn run_next(&self) {
        if let Some(next) = self.find_next() {
            let (cur_tcx_ptr, nxt_tcx_ptr) = {
                let mut inner = self.inner.exclusive_access();
                let current = inner.current;

                debug!("Executing App {next}");

                inner.tasks[next].status = TaskStatus::Running;
                inner.current = next;

                (
                    &mut inner.tasks[current].context as *mut TaskContext,
                    &inner.tasks[next].context as *const TaskContext,
                )
            };

            _switch(cur_tcx_ptr, nxt_tcx_ptr);
        } else {
            info!("All Tasks Completed!");
            print_switch_cost();
            shutdown(false);
        }
    }

    fn run_first(&self) -> ! {
        let first_tcx_ptr = {
            let mut inner = self.inner.exclusive_access();
            inner.refresh_current_time();

            let first = &mut inner.tasks[0];
            info!("Executing App 0");
            first.status = TaskStatus::Running;
            &first.context as *const TaskContext
        };

        let mut null_tcx = TaskContext::zero_init();

        _switch(&mut null_tcx as *mut TaskContext, first_tcx_ptr);

        unreachable!()
    }
}

struct TaskManagerInner {
    tasks: [TaskControlBlock; MAX_APP_NUM],
    current: usize,
    time: Time,
}

impl TaskManagerInner {
    pub fn refresh_current_time(&mut self) -> Duration {
        self.time.elapsed_and_update()
    }
}

pub fn run_first() {
    TASK_MANAGER.run_first();
}

pub fn suspend_current_then_run_next() {
    TASK_MANAGER.mark_current_status(TaskStatus::Ready);
    TASK_MANAGER.run_next();
}

pub fn exit_current_then_run_next() {
    TASK_MANAGER.mark_current_status(TaskStatus::Exit);
    TASK_MANAGER.print_current_tcb();
    TASK_MANAGER.run_next();
}

pub fn calc_user_time() {
    TASK_MANAGER.calc_user_time();
}

pub fn calc_kernel_time() {
    TASK_MANAGER.calc_kernel_time();
}
