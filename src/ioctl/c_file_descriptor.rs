use super::libc::{c_int, close};
use std::fmt;


pub struct CFileDescriptor {
    fd: c_int
}


impl CFileDescriptor {
    pub fn new(fd: c_int) -> CFileDescriptor {
        CFileDescriptor {fd: fd}
    }
}


impl Drop for CFileDescriptor {
    fn drop(&mut self) {
        let result = unsafe {
            close(self.fd)
        };

        // There isn't much we can do if this failed
        if result != 0 {
            println!("Warning: failed to close file descriptor when dropping");
        }
    }
}


impl fmt::Show for CFileDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<C File Descriptor {}>", self.fd)
    }
}
