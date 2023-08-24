use crate::sys;

pub struct Buffer<const N: usize> {
    buf: [u8; N],
    len: usize,
}

impl<const N: usize> Buffer<N> {
    pub fn new() -> Self {
        Buffer {
            buf: [0; N],
            len: 0,
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.buf[..self.len]
    }
}

impl<const N: usize> core::fmt::Write for Buffer<N> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for &b in s.as_bytes() {
            if self.len < N {
                self.buf[self.len] = b;
                self.len += 1;
            } else {
                return Err(core::fmt::Error);
            }
        }
        Ok(())
    }
}

pub fn print_bytes(bytes: &[u8]) {
    sys::write(sys::STDOUT, bytes);
}
