//! Process management related functions.

use psp2_sys::*;

#[repr(u32)]
pub enum PowerTick {
    /// Cancel all timers.
    Default = SCE_KERNEL_POWER_TICK_DEFAULT,
    /// Cancel automatic suspension timer.
    DisableAutoSuspend = SCE_KERNEL_POWER_TICK_DISABLE_AUTO_SUSPEND,
    /// Cancel OLED-off timer.
    DisableOLEDOff = SCE_KERNEL_POWER_TICK_DISABLE_OLED_OFF,
    /// Cancel OLED dimming timer.
    DisableOLEDDimming = SCE_KERNEL_POWER_TICK_DISABLE_OLED_DIMMING
}

/// Exit the current process with specified return code.
pub fn exit_process(code: i32) -> ! {
    unsafe {
        sceKernelExitProcess(code);
    }
    loop {}
}

/// Cancel the specified idle timers. Use repeatedly to prevent the
/// device from entering power save.
pub fn power_tick(ty: PowerTick) {
    unsafe {
        sceKernelPowerTick(ty as i32);
    }
}

/// Get the kernel process time of the current process.
pub fn get_process_time() -> u64 {
    unsafe {
        sceKernelGetProcessTimeWide()
    }
}

/// Get the lower 32 bits of process time of the current process.
pub fn get_process_time_low() -> u32 {
    unsafe {
        sceKernelGetProcessTimeLow()
    }
}

/// Get the current process's process ID.
pub fn get_process_id() -> i32 {
    unsafe {
        sceKernelGetProcessId()
    }
}