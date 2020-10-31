pub struct Consumer<Message> {
    receiver: std::sync::mpsc::Receiver<Message>,
    behavior: fn(&mut Message) -> std::io::Result<()>,
}

pub struct ConsumerHandle<Message> {
    sender: std::sync::mpsc::SyncSender<Message>,
    join_handle: std::thread::JoinHandle<Message>,
}

impl<Message> ConsumerHandle<Message>
where Message: Send {
    pub fn build<F>(behavior: F) -> std::io::Result<Self>
    where F: std::ops::Fn(Message) -> std::io::Result<()>,
          F: Send,
          F: Sync,
          F: 'static,
          Message: 'static {
        let (sender, receiver) = std::sync::mpsc::sync_channel(4096);
        let join_handle = std::thread::spawn(move || loop {
            if let Ok(message) = receiver.recv() { behavior(message); }
        });
        Ok(ConsumerHandle {
            sender,
            join_handle,
        })

    }

    pub fn send(&mut self, message: Message) -> std::result::Result<(), std::sync::mpsc::SendError<Message>> {
        self.sender.send(message)
    }
}
