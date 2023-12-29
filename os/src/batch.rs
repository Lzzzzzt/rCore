use core::mem::size_of;

use lazy_static::lazy_static;
use log::{debug, info};

use crate::{sbi::shutdown, sync::UPSafeCell, trap::context::TrapContext};

const MAX_APP_NUM: usize = 16;
const APP_BASE_ADDRESS: usize = 0x80400000;
const APP_SIZE_LIMIT: usize = 0x20000;

const KERNEL_STACK_SIZE: usize = 1 << 12;
const USER_STACK_SIZE: usize = 1 << 12;

lazy_static! {
    static ref APP_MAMAGER: UPSafeCell<AppManager> = unsafe {
        UPSafeCell::new({
            extern "C" {
                fn _num_app();
            }

            let num_app_ptr = _num_app as usize as *const usize;
            let num_app = num_app_ptr.read_volatile();
            let mut app_start = [0usize; MAX_APP_NUM + 1];
            let app_start_raw = core::slice::from_raw_parts(num_app_ptr.add(1), num_app + 1);
            app_start[..=num_app].copy_from_slice(app_start_raw);

            AppManager {
                num_app,
                current_app: 0,
                app_start,
            }
        })
    };
}

static mut KERNEL_STACK: KernelStack = KernelStack {
    data: [0; KERNEL_STACK_SIZE],
};
static mut USER_STACK: UserStack = UserStack {
    data: [0; USER_STACK_SIZE],
};

struct AppManager {
    num_app: usize,
    current_app: usize,
    app_start: [usize; MAX_APP_NUM + 1],
}

impl AppManager {
    unsafe fn load_app(&self, app_id: usize) {
        if self.num_app <= app_id {
            info!("All Applications Completed");
            shutdown();
        }

        debug!("Loading app_{}", app_id);

        // clear app memory spcae
        core::slice::from_raw_parts_mut(APP_BASE_ADDRESS as *mut u8, APP_SIZE_LIMIT).fill(0);

        let app_src = core::slice::from_raw_parts(
            self.app_start[app_id] as *const u8,
            self.app_start[app_id + 1] - self.app_start[app_id],
        );
        let app_dst = core::slice::from_raw_parts_mut(APP_BASE_ADDRESS as *mut u8, app_src.len());
        app_dst.copy_from_slice(app_src);

        core::arch::asm!("fence.i");
    }

    pub fn get_current_app(&self) -> usize {
        self.current_app
    }

    pub fn move_to_next_app(&mut self) {
        self.current_app += 1;
    }

    pub fn print_app_info(&self) {
        debug!("App Number = {}", self.num_app);

        for i in 0..self.num_app {
            debug!(
                " app_{} [{:#x}, {:#x})",
                i,
                self.app_start[i],
                self.app_start[i + 1]
            );
        }
    }
}

#[repr(C)]
struct KernelStack {
    data: [u8; KERNEL_STACK_SIZE],
}

impl KernelStack {
    fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + USER_STACK_SIZE
    }

    pub fn push_context(&self, cx: TrapContext) -> &'static mut TrapContext {
        let cx_ptr = (self.get_sp() - size_of::<TrapContext>()) as *mut TrapContext;
        unsafe { *cx_ptr = cx };
        unsafe { cx_ptr.as_mut().unwrap() }
    }
}

#[repr(C)]
struct UserStack {
    data: [u8; USER_STACK_SIZE],
}

impl UserStack {
    fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + USER_STACK_SIZE
    }
}

pub fn run_next_app() -> ! {
    {
        let mut manager = APP_MAMAGER.exclusive_access();

        let cur_app = manager.get_current_app();

        unsafe { manager.load_app(cur_app) };

        manager.move_to_next_app();
    }

    extern "C" {
        fn __ret_to_user(cx_addr: usize);
    }

    let cx = unsafe {
        KERNEL_STACK.push_context(TrapContext::init(APP_BASE_ADDRESS, USER_STACK.get_sp()))
    };

    unsafe { __ret_to_user(cx as *const _ as usize) }

    unreachable!()
}

pub fn init() {
    APP_MAMAGER.exclusive_access().print_app_info();
}

pub fn check_address_range(addr: *const u8, len: usize) -> bool {
    let manager = APP_MAMAGER.exclusive_access();

    let cur_app = manager.get_current_app() - 1;

    let app_len = manager.app_start[cur_app + 1] - manager.app_start[cur_app];

    let cur_app_space_start = APP_BASE_ADDRESS;
    let cur_app_space_end = APP_BASE_ADDRESS + app_len;

    let start = addr as usize;
    let end = start + len;
    let ksp = unsafe { KERNEL_STACK.get_sp() };
    let usp = unsafe { USER_STACK.get_sp() };

    let ks_range = ksp..(KERNEL_STACK_SIZE + ksp);
    let us_range = usp..(USER_STACK_SIZE + usp);
    let user_range = cur_app_space_start..cur_app_space_end;

    (ks_range.contains(&start) && ks_range.contains(&end))
        || (us_range.contains(&start) && us_range.contains(&end))
        || (user_range.contains(&start) && user_range.contains(&end))
}
