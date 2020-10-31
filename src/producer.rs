use std::net::SocketAddr;

use super::listener::Listener;

pub struct Producer<Message> {
    channel: std::sync::mpsc::SyncSender<(SocketAddr, Message)>,
    transformation: fn(&[u8]) -> std::io::Result<Message>,
    listener: Listener,
}

impl<Message> Producer<Message> {
    pub fn build<A>(
        addr: A,
        channel: std::sync::mpsc::SyncSender<(SocketAddr, Message)>,
        transformation: fn(&[u8]) -> std::io::Result<Message>,
    ) -> std::io::Result<Self>
    where
        A: std::net::ToSocketAddrs,
    {
        let listener = Listener::build(addr)?;
        Ok(Producer {
            channel,
            transformation,
            listener,
        })
    }

    pub fn pump(&mut self) -> ! {
        loop {
            match self.listener.recv_next() {
                Err(err) => println!("Error receiving: {:?}", err),
                Ok(()) => match self.listener.recv_from_response() {
                    Some((bytes_read, from_addr)) => {
                        match (self.transformation)(&self.listener.buffer()[..bytes_read]) {
                            Ok(message) => {
                                if let Err(err) = self.channel.send((from_addr, message)) {
                                    println!("Error sending message through channel: {:?}", err);
                                }
                            }
                            Err(_) => {
                                println!("Could not parse message: {:?}", bytes_read);
                            }
                        }
                    }
                    None => unreachable!("impossible, called recv_next successfully"),
                },
            }
        }
    }
}
