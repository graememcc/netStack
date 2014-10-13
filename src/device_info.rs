use std::fmt;


static DEFAULTDEVICE: &'static str = "/dev/net/tun";
static DEFAULTINTERFACE: &'static str = "tap0";


pub enum CloneDevice {
    DefaultDevice,
    UserDefinedDevice(String)
}


impl fmt::Show for CloneDevice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DefaultDevice => write!(f, "{}", DEFAULTDEVICE),
            UserDefinedDevice(ref d) => write!(f, "{}", d),
        }
    }
}


pub enum InterfaceName {
    DefaultInterface,
    UserDefinedName(String)
}


impl fmt::Show for InterfaceName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DefaultInterface => write!(f, "{}", DEFAULTINTERFACE),
            UserDefinedName(ref i) => write!(f, "{}", i),
        }
    }
}


pub struct DeviceInfo {
    clone_device: CloneDevice,
    interface: InterfaceName
}


impl DeviceInfo {
    pub fn new(d: CloneDevice, i: InterfaceName) -> DeviceInfo {
        DeviceInfo {clone_device: d, interface: i}
    }


    pub fn get_clone_device_name(&self) -> String {
        format!("{}", self.clone_device)
    }


    pub fn get_interface_name(&self) -> String {
        format!("{}", self.interface)
    }
}


impl fmt::Show for DeviceInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "clone device: {}, interface name: {}", self.clone_device, self.interface)
    }
}
