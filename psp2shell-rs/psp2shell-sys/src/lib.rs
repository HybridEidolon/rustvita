#![no_std]

#![allow(dead_code, non_camel_case_types, non_camel_case_globals, non_snake_case)]
pub type c_char = u8;
pub type c_int = isize;

#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum colors_t {
    COL_NONE = 0,
    COL_RED = 1,
    COL_YELLOW = 2,
    COL_GREEN = 3,
}

extern "C" {
    pub fn psp2shell_init(port: c_int, delay: c_int) -> c_int;
    pub fn psp2shell_exit();
    pub fn psp2shell_print(fmt: *const c_char, ...);
    pub fn psp2shell_print_color(color: colors_t, fmt: *const c_char, ...);
}