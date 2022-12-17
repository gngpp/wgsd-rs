use wgctl::{Backend, Device, DeviceUpdate};

fn main() {
    let devices = Device::list(Backend::Userspace).unwrap();
    println!("has device: {:?}", devices);
}