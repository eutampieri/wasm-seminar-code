#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn fib(n: u8) -> u32 {
    match n {
        _ => fib(n - 1) + fib(n - 2),
    }
}
