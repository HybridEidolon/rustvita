#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(collections)]
#![feature(alloc)]
#![link(name = "SceKernel_stub")]

extern crate collections;
extern crate alloc;

#[doc(hidden)]
pub mod psp2_sys;

pub mod libc;
pub mod kernel;

use core::fmt;

use kernel::process;
use kernel::thread;

#[no_mangle]
pub extern "C" fn main(_: isize, _: *const *const u8) -> isize {
    for _ in 0..5 {
        process::power_tick(process::PowerTick::Default);
        thread::delay_thread(1_000_000);
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
