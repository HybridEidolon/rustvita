//! Physical buttons.

use psp2_sys::*;

pub struct CtrlData {
    pub time_stamp: u64,
    pub buttons: u32,
    pub lx: u8,
    pub ly: u8,
    pub rx: u8,
    pub ry: u8
}

impl CtrlData {
    #[inline] pub fn circle(&self) -> bool { self.buttons & SCE_CTRL_CIRCLE > 0 }
    #[inline] pub fn cross(&self) -> bool { self.buttons & SCE_CTRL_CROSS > 0 }
    #[inline] pub fn square(&self) -> bool { self.buttons & SCE_CTRL_SQUARE > 0 }
    #[inline] pub fn triangle(&self) -> bool { self.buttons & SCE_CTRL_TRIANGLE > 0 }
    #[inline] pub fn start(&self) -> bool { self.buttons & SCE_CTRL_START > 0 }
    #[inline] pub fn select(&self) -> bool { self.buttons & SCE_CTRL_SELECT > 0 }
    #[inline] pub fn up(&self) -> bool { self.buttons & SCE_CTRL_UP > 0 }
    #[inline] pub fn down(&self) -> bool { self.buttons & SCE_CTRL_DOWN > 0 }
    #[inline] pub fn left(&self) -> bool { self.buttons & SCE_CTRL_LEFT > 0 }
    #[inline] pub fn right(&self) -> bool { self.buttons & SCE_CTRL_RIGHT > 0 }
    #[inline] pub fn ltrigger(&self) -> bool { self.buttons & SCE_CTRL_LTRIGGER > 0 }
    #[inline] pub fn rtrigger(&self) -> bool { self.buttons & SCE_CTRL_RTRIGGER > 0 }
}

pub fn peek_buffer_positive() -> CtrlData {
    let mut r = SceCtrlData::default();
    unsafe {
        sceCtrlPeekBufferPositive(0, &mut r, 1);
    }
    CtrlData {
        time_stamp: r.timeStamp,
        buttons: r.buttons,
        lx: r.lx,
        ly: r.ly,
        rx: r.rx,
        ry: r.ry
    }
}