//! Thread related functions.

use psp2_sys::*;

/// Delay the current thread by the given microseconds.
pub fn delay_thread(delay: u32) {
    unsafe {
        sceKernelDelayThread(delay);
    }
}

/// Delay the current thread, handling any callbacks.
pub fn delay_thread_cb(delay: u32) {
    unsafe {
        sceKernelDelayThreadCB(delay);
    }
}

/// Get the current thread's ID.
pub fn get_thread_id() -> i32 {
    unsafe {
        sceKernelGetThreadId()
    }
}