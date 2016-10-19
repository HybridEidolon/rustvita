#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(collections)]
#![feature(alloc)]
#![feature(compiler_builtins_lib)]

#[macro_use] extern crate collections;
extern crate alloc;

extern crate compiler_builtins;

// Link module stubs

#[link(name = "SceKernel_stub")]
extern {}

#[link(name = "SceCtrl_stub")]
extern {}

#[link(name = "SceAudio_stub")]
extern {}

// System imports
#[doc(hidden)]
pub mod psp2_sys;
#[doc(hidden)]
pub mod psp2_shaders;

pub mod libc;

pub mod kernel;
pub mod ctrl;
pub mod audioout;

use core::fmt;
use core::iter::Iterator;
use collections::Vec;
use alloc::boxed::Box;

use kernel::process;
//use kernel::thread;

#[no_mangle]
pub extern "C" fn main(_: isize, _: *const *const u8) -> isize {
    let port = audioout::Port::open(audioout::PortType::Main, 512, 48000, audioout::Mode::Mono);
    let mut buf = vec![0; port.buf_size()].into_boxed_slice();
    let buf: Box<[u8]> = (0..port.buf_size())
        .enumerate()
        .map(|a| {a.1 as u8})
        .collect::<Vec<_>>().into_boxed_slice();
    loop {
        process::power_tick(process::PowerTick::Default);
        port.output(&buf);
        let a = ctrl::peek_buffer_positive();
        if a.triangle() {
            break
        }
    }
    process::exit_process(0);
}

#[lang = "panic_fmt"]
extern fn panic_fmt(_fmtargs: fmt::Arguments, _file: &'static str, _line: u32) -> ! {
    let mut i = 0;
    loop {
        i += 1;
        if i > 10000000 {
            break
        }
    }
    process::exit_process(1);
}

#[lang = "eh_personality"]
extern fn eh_personality() {

}
