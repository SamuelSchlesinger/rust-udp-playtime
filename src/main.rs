mod listener;
mod producer;
mod consumer;

use producer::Producer;
use consumer::ConsumerGroup;

fn from_utf8(buffer: &[u8]) -> Option<String> {
    let os = std::str::from_utf8(buffer);
    match os {
        Err(_) => None,
        Ok(s) => Some(String::from(s)),
    }
}

fn main() -> ! {
    let consumer_group = ConsumerGroup::build(|x: (std::net::SocketAddr, String)| {
        println!("Message received: {:?}", x.1);
        Ok(())
    }, 12).expect("tried to build a consumer");
    let mut producer =
        Producer::build(("127.0.0.1", 9018 as u16), consumer_group, from_utf8).expect("Couldn't make producer");
    // test udp client
    std::thread::spawn(|| {
        let mut n = 0;
        let socket =
            std::net::UdpSocket::bind("127.0.0.1:9019").expect("could not bind to the address");
        socket
            .connect("127.0.0.1:9018")
            .expect("could not connect to the producer");
        loop {
            n += 1;
            if let Err(err) = socket.send(&[66, 67, 68]) {
                println!("Error sending bytes: {:?}", err);
            }
            println!("Send number {}", n);
        }
    });
    // pump out translated UDP requests into the channel
    producer.pump()
}
