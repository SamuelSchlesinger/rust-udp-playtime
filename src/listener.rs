pub struct Listener {
    buffer: [u8; 4096],
    socket: std::net::UdpSocket,
    recv_from_response: Option<(usize, std::net::SocketAddr)>,
}

impl Listener {
    pub fn build<A>(addr: A) -> std::io::Result<Self>
    where
        A: std::net::ToSocketAddrs,
    {
        let socket = std::net::UdpSocket::bind(addr)?;
        Ok(Listener {
            socket,
            buffer: [0; 4096],
            recv_from_response: None,
        })
    }

    pub fn recv_next(&mut self) -> std::io::Result<()> {
        self.recv_from_response = Some(self.socket.recv_from(&mut self.buffer)?);
        Ok(())
    }

    pub fn buffer(&self) -> &[u8; 4096] {
        &self.buffer
    }

    pub fn recv_from_response(&self) -> Option<(usize, std::net::SocketAddr)> {
        self.recv_from_response
    }
}
