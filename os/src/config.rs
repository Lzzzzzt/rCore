pub const MAX_APP_NUM: usize = 16;
pub const APP_BASE_ADDRESS: usize = 0x80400000;
pub const APP_SIZE_LIMIT: usize = 0x20000;

pub const KERNEL_STACK_SIZE: usize = 1 << 12;
pub const USER_STACK_SIZE: usize = 1 << 12;

pub const CLOCK_FREQ: u64 = 10000000;

pub const KERNEL_HEAP_SIZE: usize = 0x30_0000;
