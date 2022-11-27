extern crate ipnet;
pub mod model;

pub trait NodeProperty {

    fn description(&self) -> String;

    fn mtu(&self) -> String;
    
}

#[cfg(test)]
mod tests {

    #[test]
    fn feature() {}

    #[test]
    fn ipnet_test() {}
}
