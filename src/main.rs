extern crate getopts;
use getopts::{optopt, optflag, OptGroup};
use std::os;


use device_info::{DefaultDevice, DefaultInterface, UserDefinedDevice, UserDefinedName, DeviceInfo};
mod device_info;
mod ioctl;


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
    let opts = [
        optopt("d", "device", "set the cloning device", "<cloning device>"),
        optflag("h", "help", "print this help"),
        optopt("i", "interface", "set the interface name", "<interface name>")
    ];

    let args = os::args();
    let prog_name = args[0].as_slice();

    match parse_args(opts, args.as_slice()) {
        HelpRequested => {
            println!("{}", getopts::usage(format!("{}: A virtual ethernet device creator", prog_name).as_slice(), opts));
        },

        CommandLineError(e) => {
            println!("{}", e);
        },

        DevicesObtained(d) => {
            match ioctl::get_tap_descriptor(&d) {
                ioctl::FDOpened(fd) => {
                    println!("Opened tap device successfully: {}", fd);
                    std::io::timer::sleep(std::time::duration::Duration::seconds(30));
                },
                err @ _ => {
                    println!("{}", err);
                }
            };
        }
    };
}
