mod listener;
mod producer;

use producer::Producer;

fn from_utf8(buffer: &[u8]) -> std::io::Result<String> {
    let os = std::str::from_utf8(buffer);
    match os {
        Err(_) => Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "utf8 parsing error",
        )),
        Ok(s) => Ok(String::from(s)),
    }
}

fn main() -> ! {
    let (sender, receiver) = std::sync::mpsc::sync_channel(4096);
    let mut producer =
        Producer::build(("127.0.0.1", 9018 as u16), sender, from_utf8).expect("Couldn't make producer");
    // test channel client
    std::thread::spawn(move || loop {
        let _s = receiver.recv().unwrap();
        // println!("Message received: {:?}", s);
    });
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
            if n % 100000 == 0 {
                println!("Send number {}", n);
            }
        }
    });
    // pump out translated UDP requests into the channel
    producer.pump()
}
