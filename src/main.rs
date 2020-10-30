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
    let (sender, receiver) = std::sync::mpsc::channel();
    if let Ok(mut producer) = Producer::new("127.0.0.1", 9018, sender, from_utf8) {
        std::thread::spawn(move || loop {
            let s = receiver.recv().unwrap();
            println!("Message received: {:?}", s);
        });
        std::thread::spawn(|| loop {
            let socket =
                std::net::UdpSocket::bind("127.0.0.1:9019").expect("could not bind to the address");
            socket
                .connect("127.0.0.1:9018")
                .expect("could not connect to the producer");
            socket.send(&[66, 67, 68]).expect("could not send bytes");
        });
        producer.pump()
    } else {
        panic!("Whoops")
    }
}
