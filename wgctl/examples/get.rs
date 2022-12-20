use wgctl::{Backend, Device, InterfaceName};

#[cfg(target_os = "linux")]
const BACKEND: Backend = Backend::Kernel;
#[cfg(not(target_os = "linux"))]
const BACKEND: Backend = Backend::Userspace;

fn main() {
    let interface = "test".parse::<InterfaceName>().unwrap();

    let device = Device::get(&interface, BACKEND).unwrap();
    println!("{:?}", device);
}
