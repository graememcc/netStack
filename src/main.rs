extern crate getopts;
use getopts::{optopt, optflag, OptGroup};
use std::os;


use device_info::{DefaultDevice, DefaultInterface, UserDefinedDevice, UserDefinedName, DeviceInfo};
mod device_info;


enum ParseArgsResult {
    HelpRequested,
    DevicesObtained(DeviceInfo),
    CommandLineError(getopts::Fail_)
}


// Returns either a Devices struct filled either with the default values, or None if the help option is present
fn parse_args(opts: &[OptGroup], args: &[String]) -> ParseArgsResult {
    match getopts::getopts(args, opts) {
        Err(e) => CommandLineError(e),

        Ok(options) => {
            if options.opt_present("h") {
                HelpRequested
            } else {
                DevicesObtained(DeviceInfo::new(match options.opt_str("d") {
                                                    Some(s) => UserDefinedDevice(s),
                                                    None => DefaultDevice
                                                },
                                                match options.opt_str("i") {
                                                    Some(s) => UserDefinedName(s),
                                                    None => DefaultInterface,
                                                }))
            }
        }
    }
}


fn main() {
    println!("Hello, world!")
}
