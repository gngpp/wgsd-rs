use crate::{DeviceUpdate, InterfaceName, InvalidInterfaceName, Key, KeyPair, PeerConfigBuilder};
use ipnet::IpNet;
use std::io;

pub struct WgQuick {
    interface: InterfaceName,
    cidr: Vec<IpNet>,
    mtu: u32,
    public_key: Option<Key>,
    private_key: Option<Key>,
    listen_port: Option<u16>,
    peers: Vec<PeerConfigBuilder>,
    replace_peers: bool,
}

impl WgQuick {
    pub fn new(name: &str) -> Result<WgQuick, InvalidInterfaceName> {
        let interface = <InterfaceName as std::str::FromStr>::from_str(name)?;
        Ok(Self {
            interface,
            cidr: vec![],
            mtu: 1420,
            public_key: None,
            private_key: None,
            listen_port: None,
            peers: vec![],
            replace_peers: false,
        })
    }

    pub fn set_keypair(self, keypair: KeyPair) -> Self {
        self.set_public_key(keypair.public)
            .set_private_key(keypair.private)
    }

    pub fn set_public_key(mut self, key: Key) -> Self {
        self.public_key = Some(key);
        self
    }

    pub fn set_address(mut self, address: IpNet) -> Self {
        self.cidr.push(address);
        self
    }

    pub fn set_address_list(mut self, address_list: &[IpNet]) -> Self {
        self.cidr.extend_from_slice(address_list);
        self
    }

    pub fn unset_public_key(self) -> Self {
        self.set_public_key(Key::zero())
    }

    pub fn set_private_key(mut self, key: Key) -> Self {
        self.private_key = Some(key);
        self
    }

    pub fn unset_private_key(self) -> Self {
        self.set_private_key(Key::zero())
    }

    pub fn set_listen_port(mut self, port: u16) -> Self {
        self.listen_port = Some(port);
        self
    }

    pub fn randomize_listen_port(self) -> Self {
        self.set_listen_port(0)
    }

    pub fn add_peer(mut self, peer: PeerConfigBuilder) -> Self {
        self.peers.push(peer);
        self
    }

    pub fn add_peer_with(
        self,
        pubkey: &Key,
        builder: impl Fn(PeerConfigBuilder) -> PeerConfigBuilder,
    ) -> Self {
        self.add_peer(builder(PeerConfigBuilder::new(pubkey)))
    }

    pub fn add_peers(mut self, peers: &[PeerConfigBuilder]) -> Self {
        self.peers.extend_from_slice(peers);
        self
    }

    pub fn replace_peers(mut self) -> Self {
        self.replace_peers = true;
        self
    }

    pub fn remove_peer_by_key(self, public_key: &Key) -> Self {
        let mut peer = PeerConfigBuilder::new(public_key);
        peer.remove_me = true;
        self.add_peer(peer)
    }

    pub fn apply(self, backend: crate::Backend) -> io::Result<()> {
        let mut update = DeviceUpdate::new();

        if let Some(listen_port) = self.listen_port {
            update = update.set_listen_port(listen_port);
        }

        if let Some(private_key) = self.private_key {
            update = update.set_private_key(private_key);
        }

        if let Some(public_key) = self.public_key {
            update = update.set_public_key(public_key);
        }

        if self.replace_peers {
            update = update.replace_peers();
        }

        update
            .add_peers(self.peers.as_slice())
            .apply(&self.interface, backend)?;

        #[cfg(target_os = "linux")]
        use crate::tools::linux as platform;

        #[cfg(target_os = "macos")]
        use crate::tools::macos as platform;

        platform::set_up(&self.interface, self.mtu)?;
        for address in self.cidr {
            platform::set_addr(&self.interface, address)?;
            platform::add_route(&self.interface, address)?;
        }

        Ok(())
    }
}
