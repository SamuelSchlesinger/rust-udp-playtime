pub struct Listener {
    buffer: [u8; 4096],
    socket: std::net::UdpSocket,
    bytes_read: Option<usize>,
}

impl Listener {
    pub fn new(host: &'_ str, port: u16) -> std::io::Result<Self> {
        let socket = std::net::UdpSocket::bind((host, port))?;
        Ok(Listener {
            socket,
            buffer: [0; 4096],
            bytes_read: None,
        })
    }

    pub fn recv_next(&mut self) -> std::io::Result<()> {
        self.bytes_read = Some(self.socket.recv(&mut self.buffer)?);
        Ok(())
    }

    pub fn buffer(&self) -> &[u8; 4096] {
        &self.buffer
    }

    pub fn bytes_read(&self) -> Option<usize> {
        self.bytes_read
    }
}
