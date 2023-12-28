use crate::{println, sbi::shutdown};

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!(
            "Panicked at {}:{} {}",
            location.file(),
            location.line(),
            info.message().unwrap()
        )
    } else {
        println!("Panicked at:{}", info.message().unwrap());
    }

    shutdown()
}
