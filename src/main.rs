#![feature(const_generics)]

mod consumer;
mod listener;
mod producer;

use consumer::ConsumerGroup;
use producer::Producer;

fn from_utf8(buffer: &[u8]) -> Option<String> {
    match std::str::from_utf8(buffer) {
        Err(_) => None,
        Ok(s) => Some(String::from(s)),
    }
}

fn main() -> ! {
    use std::sync::{atomic::AtomicUsize, Arc};
    let consumer_counter = Arc::new(AtomicUsize::new(0));
    let cc = consumer_counter.clone();
    let behavior = Arc::new(
        move |x: (std::net::SocketAddr, String),
              env: &mut std::sync::Arc<std::sync::atomic::AtomicUsize>| {
            let e = env.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            if e % 10000 == 0 {
                println!("{}", e);
            }
            Ok(())
        },
    );
    let consumer_group = ConsumerGroup::build(behavior.clone(), &consumer_counter, 1)
        .expect("tried to build a consumer");
    let mut producer: Producer<String, Arc<AtomicUsize>, 4096> = Producer::build(("127.0.0.1", 9018 as u16), consumer_group, from_utf8)
        .expect("Couldn't make producer");
    // test udp client
    std::thread::spawn(move || {
        let mut n = 0;
        let socket =
            std::net::UdpSocket::bind("127.0.0.1:9019").expect("could not bind to the address");
        socket
            .connect("127.0.0.1:9018")
            .expect("could not connect to the producer");
        loop {
            n += 1;
            if let Err(err) = socket.send(format!("{}", n).as_bytes()) {
                println!("Error sending bytes: {:?}", err);
            }
            std::thread::yield_now();
            let m = cc.load(std::sync::atomic::Ordering::SeqCst);
            if n % 10000 == 0 {
                println!(
                    "Sent: {}, Received: {}, Sent - Received: {}",
                    n,
                    m,
                    n - m
                );
            }
        }
    });
    // pump out translated UDP requests into the channel
    producer.pump()
}
