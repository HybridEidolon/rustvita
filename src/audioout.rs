use psp2_sys::*;

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

impl Port {
    /// Creates a new audio output port with the given properties.
    pub fn open(len: u32, freq: u32, mode: Mode) -> Port {
        let port = unsafe {
            match sceAudioOutOpenPort(SCE_AUDIO_OUT_PORT_TYPE_MAIN as i32, len as i32, freq as i32, mode as i32) {
                i if i < 0 => panic!("failed to open audio port"),
                i => i
            }
        };
        unsafe {
            sceAudioOutSetConfig(port, -1, -1, -1);
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
    pub fn output(&self, buf: &[u8]) {
        if buf.len() != self.buf_size() {
            panic!("invalid buffer size");
        }
        unsafe {
            sceAudioOutOutput(self.port, buf as *const _ as *const ::libc::c_void);
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