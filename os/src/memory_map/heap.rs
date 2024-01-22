use core::alloc::Layout;

use buddy_system_allocator::LockedHeap;

use crate::config::KERNEL_HEAP_SIZE;

#[global_allocator]
static HEAP_ALLOCATOR: LockedHeap<32> = LockedHeap::empty();

static mut HEAP: [u8; KERNEL_HEAP_SIZE] = [0; KERNEL_HEAP_SIZE];

pub fn init() {
    unsafe {
        HEAP_ALLOCATOR
            .lock()
            .init(HEAP.as_ptr() as usize, KERNEL_HEAP_SIZE)
    }
}

#[alloc_error_handler]
pub fn alloc_error_handler(layout: Layout) -> ! {
    panic!("Heap Allocate Error: Layout = {:?}", layout)
}
