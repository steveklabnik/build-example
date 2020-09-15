#![feature(start)]
#![no_std]

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    kernel::entry();

    0
}


#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
