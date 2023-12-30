#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TaskContext {
    ra: usize,
    sp: usize,
    s: [usize; 12],
}

impl TaskContext {
    pub fn zero_init() -> Self {
        Default::default()
    }

    pub fn goto_ret_to_user(kernel_stack_p: usize) -> Self {
        extern "C" {
            fn __trap_return();
        }
        Self {
            ra: __trap_return as usize,
            sp: kernel_stack_p,
            s: [0; 12],
        }
    }
}
