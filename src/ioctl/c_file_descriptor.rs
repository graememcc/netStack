use super::libc::{c_int, close, read, c_void, size_t};
use std::fmt;
use std::io::{Reader, IoResult, IoError, standard_error, EndOfFile};


pub struct CFileDescriptor {
    fd: c_int
}


impl CFileDescriptor {
    pub fn new(fd: c_int) -> CFileDescriptor {
        CFileDescriptor {fd: fd}
    }
}


impl Reader for CFileDescriptor {
    fn read(&mut self, buf: &mut [u8]) -> IoResult<uint> {
        let buf_len = buf.len() as i64;
        let buf_ptr = buf.as_ptr();

        unsafe {
            let bytes_read =  read(self.fd, buf_ptr as *mut c_void, buf_len as size_t);

            match bytes_read {
                0 => Err(standard_error(EndOfFile)),

                -1 => Err(IoError::last_error()),

                n @ _  => {
                    if n > buf_len {
                        // If C has screwed us by reading more than the buffer length, then we're in a bad spot. Our
                        // return address might have been overwritten on the stack, as buf would have been in our
                        // caller's stack frame.
                        fail!("Buffer tainted by C read. Unsafe to continue")
                    }

                    Ok(n as uint)
                }
            }
        }
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
