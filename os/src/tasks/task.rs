use crate::timer::Time;

use super::context::TaskContext;

#[derive(Clone, Copy, PartialEq, Default)]
pub enum TaskStatus {
    #[default]
    Uninit,
    Ready,
    Running,
    Exit,
}

#[derive(Clone, Copy, Default)]
pub struct TaskControlBlock {
    pub status: TaskStatus,
    pub context: TaskContext,
    pub user_time: Time,
    pub kernel_time: Time,
    pub id: usize,
}

impl TaskControlBlock {
    pub fn is_ready(&self) -> bool {
        self.status == TaskStatus::Ready
    }

    pub fn uninit() -> Self {
        Default::default()
    }
}
