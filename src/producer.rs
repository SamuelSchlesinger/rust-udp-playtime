use super::listener::Listener;

pub struct Producer<A> {
    channel: std::sync::mpsc::Sender<A>,
    transformation: fn(&[u8]) -> std::io::Result<A>,
    listener: Listener,
}

impl<A> Producer<A> {
    pub fn new(
        host: &str,
        port: u16,
        channel: std::sync::mpsc::Sender<A>,
        transformation: fn(&[u8]) -> std::io::Result<A>,
    ) -> std::io::Result<Self> {
        let listener = Listener::new(host, port)?;
        Ok(Producer {
            channel,
            transformation,
            listener,
        })
    }

    pub fn pump(&mut self) -> ! {
        loop {
            match self.listener.recv_next() {
                Err(err) => {
                    println!("Error receiving: {:?}", err);
                }
                Ok(()) => match self.listener.bytes_read() {
                    Some(bytes_read) => {
                        match (self.transformation)(&self.listener.buffer()[..bytes_read]) {
                            Ok(message) => match self.channel.send(message) {
                                Ok(()) => continue,
                                Err(err) => {
                                    println!("Error sending message through channel: {:?}", err);
                                }
                            },
                            Err(_) => {
                                println!("Could not parse message: {:?}", bytes_read);
                            }
                        }
                    }
                    None => panic!("impossible, called recv_next successfully"),
                },
            }
        }
    }
}
