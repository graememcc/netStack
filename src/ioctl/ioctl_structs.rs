static IFNAMSIZ: uint = 16;
static PADDINGSIZE: uint = 22;
static IFF_TAP: u16 = 2;
static IFF_NO_PI: u16 = 0x1000;


// An enum representing the possible results of trying to create a tap_ifreq struct
pub enum GetTapDescriptorResult {
    InterfaceNameTooLong,
    InterfaceNameTooShort
}


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
    use super::{tap_ifreq, InterfaceNameTooLong, InterfaceNameTooShort};


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
}
