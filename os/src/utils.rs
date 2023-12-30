use riscv::register::sstatus;

pub fn get_app_num() -> usize {
    extern "C" {
        fn _num_app();
    }
    let app_num_ptr = _num_app as usize as *const usize;
    unsafe { app_num_ptr.read_volatile() }
}

#[allow(unused)]
pub fn enable_float_ins() {
    unsafe { sstatus::set_fs(sstatus::FS::Initial) }
}

#[allow(unused)]
pub fn enable_kernel_trap() {
    unsafe { sstatus::set_sie() }
}
