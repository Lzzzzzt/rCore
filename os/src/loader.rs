use crate::config::*;
use crate::trap::context::TrapContext;
use core::{arch::asm, mem::size_of};

static mut KERNEL_STACK: [KernelStack; MAX_APP_NUM] = [KernelStack {
    data: [0; KERNEL_STACK_SIZE],
}; MAX_APP_NUM];
static mut USER_STACK: [UserStack; MAX_APP_NUM] = [UserStack {
    data: [0; USER_STACK_SIZE],
}; MAX_APP_NUM];

// #[repr(align(4096))]
#[derive(Copy, Clone)]
struct KernelStack {
    data: [u8; USER_STACK_SIZE],
}

impl KernelStack {
    fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + USER_STACK_SIZE
    }

    pub fn push_context(&self, cx: TrapContext) -> usize {
        let cx_ptr = (self.get_sp() - size_of::<TrapContext>()) as *mut TrapContext;
        unsafe { *cx_ptr = cx };
        cx_ptr as usize
    }
}

// #[repr(align(4096))]
#[derive(Copy, Clone)]
struct UserStack {
    data: [u8; USER_STACK_SIZE],
}

impl UserStack {
    fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + USER_STACK_SIZE
    }
}

pub fn load_apps() {
    extern "C" {
        fn _num_app();
    }

    let app_num_ptr = _num_app as usize as *const usize;
    let app_num = unsafe { app_num_ptr.read_volatile() };

    assert!(app_num < MAX_APP_NUM, "Too Much Apps!");

    let app_start = unsafe { core::slice::from_raw_parts(app_num_ptr.add(1), app_num + 1) };

    unsafe { asm!("fence.i") };

    for i in 0..app_num {
        let base = get_app_base(i);

        // Clear App Space
        (base..base + APP_SIZE_LIMIT).for_each(|m| unsafe { (m as *mut u8).write_volatile(0) });

        // Copy App from .data section to memory
        let app_src = unsafe {
            core::slice::from_raw_parts(
                app_start[i] as *const usize,
                app_start[i + 1] - app_start[i],
            )
        };

        let app_dst = unsafe { core::slice::from_raw_parts_mut(base as *mut usize, app_src.len()) };

        app_dst.copy_from_slice(app_src);
    }
}

fn get_app_base(id: usize) -> usize {
    APP_BASE_ADDRESS + id * APP_SIZE_LIMIT
}

pub fn init_app_context(app_id: usize) -> usize {
    unsafe {
        KERNEL_STACK[app_id].push_context(TrapContext::init(
            get_app_base(app_id),
            USER_STACK[app_id].get_sp(),
        ))
    }
}
