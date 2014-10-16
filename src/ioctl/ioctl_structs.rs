pub use super::{InterfaceNameTooLong, InterfaceNameTooShort, GetTapDescriptorResult};

static IFNAMSIZ: uint = 16;
static PADDINGSIZE: uint = 22;
static IFF_TAP: u16 = 2;
static IFF_NO_PI: u16 = 0x1000;


// A struct to pass to ioctl to create a new tap device. The only elements of the struct required are the name and
// flags
#[repr(C)]
pub struct tap_ifreq {
    ifr_name: [u8, ..IFNAMSIZ],
    ifr_flags: u16,
    ifr_padding: [u8, ..PADDINGSIZE]
}


impl tap_ifreq {
    pub fn new(interface: &str) -> Result<tap_ifreq, GetTapDescriptorResult> {
        match interface.len() {
            0 => Err(InterfaceNameTooShort),

            // We can only accept IFNAMSIZ - 1 bytes, as we need to null-terminate the name
            n @ _ if n >= IFNAMSIZ => Err(InterfaceNameTooLong),

            _ => {
                let mut ifreq = tap_ifreq {
                    ifr_name: [0, ..IFNAMSIZ],
                    ifr_flags: IFF_TAP | IFF_NO_PI,
                    ifr_padding: [0, ..PADDINGSIZE]
                };

                for (i, byte) in interface.bytes().enumerate() {
                    ifreq.ifr_name[i] = byte;
                }

                Ok(ifreq)
            }
        }
    }
}


#[cfg(test)]
mod test {
    extern crate libc;
    use self::libc::c_ulong;
    use std::mem;
    use super::{tap_ifreq, InterfaceNameTooLong, InterfaceNameTooShort, IFNAMSIZ, IFF_TAP, IFF_NO_PI};


    #[link(name="ioctl_tests")]
    extern "C" {
      fn ifr_size() -> c_ulong;
      fn ifnamesiz() -> c_ulong;
      fn ifname_offset() -> c_ulong;
      fn ifname_member_size() -> c_ulong;
      fn ifr_flags_offset() -> c_ulong;
      fn ifr_flags_size() -> c_ulong;
      fn iff_tap() -> c_ulong;
      fn iff_no_pi() -> c_ulong;
    }


    #[test]
    fn new_bails_if_interface_len_zero() {
        assert!(match tap_ifreq::new("") {
            Err(InterfaceNameTooShort) => true,
            _ => false
        });
    }


    #[test]
    fn new_bails_if_interface_len_too_long() {
        assert!(match tap_ifreq::new("123456789ABCDEF0") {
            Err(InterfaceNameTooLong) => true,
            _ => false
        });
    }


    #[test]
    fn struct_is_correct_size() {
        unsafe {
            assert!(mem::size_of::<tap_ifreq>() == ifr_size() as uint);
        }
    }


    #[test]
    fn ifnamesize_is_correct() {
        unsafe {
            assert!(IFNAMSIZ == ifnamesiz() as uint);
        }
    }


    #[test]
    fn ifname_offset_is_correct() {
        match tap_ifreq::new("IFNAME") {
            Ok(t) => {
                let base_ptr: *const tap_ifreq = &t;
                let base = base_ptr.to_uint();

                let name_ptr: *const [u8, ..IFNAMSIZ] = &t.ifr_name;

                unsafe {
                    assert!(name_ptr.to_uint() - base == ifname_offset() as uint);
                }
            },

            _ => {
                fail!("Failed to create struct to measure memory!");
            }
        };
    }


    #[test]
    fn ifr_name_member_size_is_correct() {
        match tap_ifreq::new("IFNAME") {
            Ok(t) => {
                unsafe {
                    assert!(mem::size_of_val(&t.ifr_name) == ifname_member_size() as uint);
                }
            },

            _ => {
                fail!("Failed to create struct to measure memory!");
            }
        };
    }


    #[test]
    fn ifr_flags_offset_is_correct() {
        match tap_ifreq::new("IFNAME") {
            Ok(t) => {
                let base_ptr: *const tap_ifreq = &t;
                let base = base_ptr.to_uint();

                let flags_ptr: *const u16 = &t.ifr_flags;

                unsafe {
                    assert!(flags_ptr.to_uint() - base == ifr_flags_offset() as uint);
                }
            },

            _ => {
                fail!("Failed to create struct to measure memory!");
            }
        };
    }


    #[test]
    fn ifr_flags_member_size_is_correct() {
        match tap_ifreq::new("IFNAME") {
            Ok(t) => {
                unsafe {
                    assert!(mem::size_of_val(&t.ifr_flags) == ifr_flags_size() as uint);
                }
            },

            _ => {
                fail!("Failed to create struct to measure memory!");
            }
        };
    }


    #[test]
    fn iff_tap_has_correct_value() {
        unsafe {
            assert!(IFF_TAP as uint == iff_tap() as uint);
        }
    }


    #[test]
    fn iff_no_pi_has_correct_value() {
        unsafe {
            assert!(IFF_NO_PI as uint == iff_no_pi() as uint);
        }
    }
}
