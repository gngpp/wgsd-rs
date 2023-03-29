use wg::{Backend, Device};

#[cfg(target_os = "linux")]
const BACKEND: Backend = Backend::Kernel;
#[cfg(not(target_os = "linux"))]
const BACKEND: Backend = Backend::Userspace;

fn main() {
    let device_name_list = Device::list(BACKEND).unwrap();
    for ifname in device_name_list {
        let device = Device::get(&ifname, BACKEND).unwrap();
        device.print().unwrap();
    }
}
