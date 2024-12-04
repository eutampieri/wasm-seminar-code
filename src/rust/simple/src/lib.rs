#![no_std]

use core::panic::PanicInfo;

mod add;
pub use add::add;
mod fact;
pub use fact::fact;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}
