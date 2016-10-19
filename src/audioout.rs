use psp2_sys::*;

#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PortType {
    Main = 0,
    Bgm = 1,
    Voice = 2
}

#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Mono = 0,
    Stereo = 1
}

pub struct Port {
    port: ::libc::c_int,
    len: u32,
    _freq: u32,
    mode: Mode
}

unsafe impl Send for Port {}

impl Port {
    /// Opens a new audio output port with the given properties.
    ///
    /// The Main port only supports 48000 sample frequency. Bgm and Voice
    /// can be opened with 8000, 11025, 12000, 16000, 22050, 24000, 32000,
    /// 44100, or 48000.
    pub fn open(port: PortType, len: u32, freq: u32, mode: Mode) -> Port {
        let port = unsafe {
            match sceAudioOutOpenPort(port as i32, len as i32, freq as i32, mode as i32) {
                i if i < 0 => panic!("failed to open audio port"),
                i => i
            }
        };
        Port {
            port: port,
            len: len,
            _freq: freq,
            mode: mode
        }
    }

    /// The size, in bytes, that an audio buffer passed to [output] must be.
    pub fn buf_size(&self) -> usize {
        self.len as usize * 2 * (if self.mode == Mode::Stereo {2} else {1})
    }

    /// Outputs the data in the given buffer to the port. The buffer must be the exact size
    /// specified by [buf_size]. Blocks until the buffer is finished playing.
    ///
    /// If the port was opened in Stereo mode, then the left and right channels are interleaved
    /// in the buffer.
    pub fn output(&self, buf: &[u8]) {
        if buf.len() != self.buf_size() {
            panic!("invalid buffer size");
        }
        unsafe {
            sceAudioOutOutput(self.port, buf as *const _ as *const ::libc::c_void);
        }
    }

    /// Set the volume of each channel independently. Pass None to preserve the previous volume.
    pub fn set_volume(&self, left: Option<u16>, right: Option<u16>) {
        let flags = if left.is_some() { 1 } else { 0 } | if right.is_some() { 2 } else { 0 };
        let mut arr = [left.unwrap_or(0) as i32, right.unwrap_or(0) as i32];
        unsafe {
            sceAudioOutSetVolume(self.port, flags, arr[..].as_mut_ptr());
        }
    }
}

impl Drop for Port {
    fn drop(&mut self) {
        unsafe {
            match sceAudioOutReleasePort(self.port) {
                i if i < 0 => panic!("failed to close audio port"),
                _ => ()
            }
        }
    }
}