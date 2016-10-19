#![no_std]

extern crate psp2shell_sys as sys;

use core::marker::PhantomData;

pub struct Shell {
    pd: PhantomData<u8>
}

impl Shell {
    /// Initializes the shell by starting a new background thread.
    /// Only ever call once in an application. We don't have synchronization primitives
    /// to ensure this doesn't happen.
    pub fn init(port: u16, delay: i32) -> Result<Shell, isize> {
        unsafe {
            let r = sys::psp2shell_init(port as isize, delay as isize);
            if r < 0 {
                return Err(r)
            }
        }
        Ok(Shell {
            pd: Default::default()
        })
    }

    pub fn print(&self, text: &str) {
        unsafe {
            sys::psp2shell_print(text.as_ptr());
        }
    }
}

impl Drop for Shell {
    fn drop(&mut self) {
        // exit psp2shell
        unsafe { sys::psp2shell_exit(); }
    }
}