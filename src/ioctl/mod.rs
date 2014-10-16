extern crate libc;

use self::c_file_descriptor::CFileDescriptor;
use self::ioctl_structs::tap_ifreq;
use std::fmt;
use self::libc::{c_int, open, O_RDWR};
use super::device_info::DeviceInfo;

pub mod c_file_descriptor;
mod ioctl_structs;


static TUNSETIFF: u64 = 0x400454ca;


// An enum representing the possible results of trying to create a tap_ifreq struct
pub enum GetTapDescriptorResult {
    FailedToOpenCloneDevice,
    InterfaceNameTooLong,
    InterfaceNameTooShort,
    IoctlFailed,
    FDOpened(CFileDescriptor)
}


impl fmt::Show for GetTapDescriptorResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FailedToOpenCloneDevice => write!(f, "{}", "Failed to open tun clone device!"),
            InterfaceNameTooLong => write!(f, "{}", "Interface name was too long!"),
            InterfaceNameTooShort => write!(f, "{}", "Interface name was too short!"),
            IoctlFailed => write!(f, "{}", "Native ioctl call failed!"),
            FDOpened(ref fd) => write!(f, "Device opened successfully: {}", fd),
        }
    }
}


// Bind the native ioctl
#[link(name="c")]
extern "C" {
    fn ioctl(fd: c_int, command: u64, ifreq: *const ioctl_structs::tap_ifreq) -> c_int;
}


// Given the tun clone device fd, ioctl to a new device
fn ioctl_for_clone(fd: c_int, interface: &str) -> Result<c_int, GetTapDescriptorResult> {
    match tap_ifreq::new(interface) {
        Ok(ifreq) => {
            unsafe {
                Ok(ioctl(fd, TUNSETIFF, &ifreq))
            }
        },

        Err(e) => Err(e)
    }
}


// Open the given device file, which is assumed to be the tun clone device
fn get_clone_device_fd(device: &str) -> c_int {
    unsafe {
        device.with_c_str(|buf| {
            open(buf, O_RDWR, 0 as u32)
        })
    }
}


pub fn get_tap_descriptor(device_info: &DeviceInfo) -> GetTapDescriptorResult {
    let device_name = device_info.get_clone_device_name();
    let clone_fd = get_clone_device_fd(device_name.as_slice());

    // Halt if we encounter a negative file descriptor
    if clone_fd == -1 {
        return FailedToOpenCloneDevice
    }

    let interface = device_info.get_interface_name();

    match ioctl_for_clone(clone_fd, interface.as_slice()) {
        Ok(ioctl_return_val) => {
            if ioctl_return_val < 0 {
                IoctlFailed
            } else {
                FDOpened(CFileDescriptor::new(clone_fd))
            }
        },

        Err(e) => e
    }
}


#[cfg(test)]
mod test {
    use super::libc::c_ulong;
    use super::TUNSETIFF;


    #[link(name="ioctl_tests")]
    extern "C" {
      fn tunsetiff() -> c_ulong;
    }


    #[test]
    fn tunsetiff_has_correct_value() {
        unsafe {
            assert!(TUNSETIFF as uint == tunsetiff() as uint);
        }
    }
}
