use wgctl::{Backend, Device, InterfaceName};

fn main() {
    let interface = "test".parse::<InterfaceName>().unwrap();

    let device = Device::get(&interface, Backend::Userspace).unwrap();
    println!("{:?}", device);
}