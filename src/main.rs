#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(collections)]
#![feature(alloc)]
//#![feature(compiler_builtins_lib)]

#[macro_use] extern crate collections;
extern crate alloc;

//extern crate compiler_builtins;

extern crate psp2shell;

// Link libraries
// Normally the sys crate would handle this but we need EXACT linkage order.

//#[link(name = "psp2shell")]
//extern {}

// Link module stubs

#[link(name = "SceKernel_stub")]
extern {}

#[link(name = "SceCtrl_stub")]
extern {}

#[link(name = "SceAudio_stub")]
extern {}

#[link(name = "SceSysmodule_stub")]
extern {}

#[link(name = "SceNet_stub")]
extern {}

#[link(name = "SceNetCtl_stub")]
extern {}

#[link(name = "SceAppMgr_stub")]
extern {}

#[link(name = "SceRtc_stub")]
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

use kernel::process;
//use kernel::thread;

use psp2shell::Shell;

#[no_mangle]
pub extern "C" fn main(_: isize, _: *const *const u8) -> isize {
    let shell = Shell::init(3333, 0).unwrap();
    real_main();
    drop(shell);
    process::exit_process(0);
}

pub fn real_main() {
    let port = audioout::Port::open(audioout::PortType::Main, 4096, 48000, audioout::Mode::Mono);

    let mut buf = vec![0u8; port.buf_size()];
    for (i, x) in buf.iter_mut().enumerate() {
        *x = i as u8;
    }

    let buf = buf.into_boxed_slice();

    loop {
        process::power_tick(process::PowerTick::Default);
        port.output(&buf);
        let a = ctrl::peek_buffer_positive();
        if a.triangle() {
            break
        }
    }
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
