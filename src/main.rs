#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(collections)]
#![feature(alloc)]

extern crate collections;
extern crate alloc;

#[link(name = "SceKernel_stub")]
extern {

}

#[link(name = "SceCtrl_stub")]
extern {

}

#[doc(hidden)]
pub mod psp2_sys;

pub mod libc;

pub mod kernel;
pub mod ctrl;

use core::fmt;

use kernel::process;
//use kernel::thread;

#[no_mangle]
pub extern "C" fn main(_: isize, _: *const *const u8) -> isize {
    loop {
        process::power_tick(process::PowerTick::Default);
        let a = ctrl::peek_buffer_positive();
        if a.triangle() {
            break
        }
    }
    process::exit_process(0);
}

#[lang = "panic_fmt"]
extern fn panic_fmt(_fmtargs: fmt::Arguments, _file: &'static str, _line: u32) -> ! {
    process::exit_process(1);
}

#[lang = "eh_personality"]
extern fn eh_personality() {

}
