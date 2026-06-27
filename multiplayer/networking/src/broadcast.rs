use bitcode::{DecodeOwned, Encode};
use network_interface::{NetworkInterface, NetworkInterfaceConfig};
use std::{
    io::ErrorKind,
    net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket},
};

pub struct LanDiscovery {
    socket: UdpSocket,
    port: u16,
    addresses: Option<Vec<IpAddr>>,
    buf: [u8; 65_535],
}

impl LanDiscovery {
    pub fn new(port: u16) -> Self {
        let socket_address = SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), port);
        let socket = UdpSocket::bind(socket_address).expect("Failed to bind");
        socket.set_nonblocking(true).expect("Failed to not block");
        socket.set_broadcast(true).expect("Failed to set broadcast");
        Self {
            socket,
            port,
            addresses: None,
            buf: [0; 65_535],
        }
    }
    pub fn broadcast<M>(&mut self, message: M)
    where
        M: Encode,
    {
        if self.addresses.is_none() {
            let network_interfaces =
                NetworkInterface::show().expect("Getting network interfaces failed!");

            let addresses: Vec<IpAddr> = network_interfaces
                .iter()
                .flat_map(|x| x.addr.iter().filter_map(|y| y.broadcast()))
                .collect();
            self.addresses = Some(addresses)
        }

        let bytes = bitcode::encode(&message);

        for &broadcast in self.addresses.as_ref().unwrap() {
            if let Err(err) = self.socket.send_to(&bytes, (broadcast, self.port)) {
                eprintln!("Broadcast to {broadcast} failed: {err}");
            }
        }
    }
    pub fn try_receive<M>(&mut self) -> Option<(IpAddr, M)>
    where
        M: DecodeOwned,
    {
        loop {
            match self.socket.recv_from(&mut self.buf) {
                Ok((len, src)) => match bitcode::decode::<M>(&self.buf[..len]) {
                    Ok(message) => return Some((src.ip(), message)),
                    Err(err) => {
                        eprintln!("Invalid UDP message from {src}: {err}");
                    }
                },

                Err(err) if err.kind() == ErrorKind::WouldBlock => return None,

                Err(err) => {
                    eprintln!("UDP receive error: {err}");
                }
            }
        }
    }
}
