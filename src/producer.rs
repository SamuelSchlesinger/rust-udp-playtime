use super::listener::Listener;
use super::consumer::ConsumerGroup;

pub struct Producer<Message> {
    consumer_group: ConsumerGroup<(std::net::SocketAddr, Message)>,
    transformation: fn(&[u8]) -> Option<Message>,
    listener: Listener,
}

impl<Message> Producer<Message>
where Message: Send {
    pub fn build<A>(
        addr: A,
        consumer_group: ConsumerGroup<(std::net::SocketAddr, Message)>,
        transformation: fn(&[u8]) -> Option<Message>,
    ) -> std::io::Result<Self>
    where
        A: std::net::ToSocketAddrs,
    {
        let listener = Listener::build(addr)?;
        Ok(Producer {
            consumer_group,
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
                        if let Some(message) =
                            (self.transformation)(&self.listener.buffer()[..bytes_read])
                        {
                            if let Err(err) = self.consumer_group.send((from_addr, message)) {
                                println!("Error sending message through channel: {:?}", err);
                            }
                        }
                    }
                    None => unreachable!("impossible, called recv_next successfully"),
                },
            }
        }
    }
}
