#![no_std]
#![no_main]
#![feature(start)]
#![feature(lang_items)]

use core::fmt;

#[no_mangle]
pub extern "C" fn main(_: isize, _: *const *const u8) -> isize {
    unsafe {
        sceKernelExitProcess(0);
    }
    0
}

#[link(name = "SceKernel_stub")]
extern {
    fn sceKernelExitProcess(code: isize) -> isize;
}

#[lang = "panic_fmt"]
extern fn panic_fmt(_fmtargs: fmt::Arguments, _file: &'static str, _line: u32) -> ! {
    loop {}
}

#[lang = "eh_personality"]
extern fn eh_personality() {

}
