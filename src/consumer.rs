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

pub struct ConsumerGroup<Message> {
    consumer_handles: Vec<ConsumerHandle<Message>>,
    last_sent: usize,
}

impl<Message> ConsumerGroup<Message>
where Message: Send {
    pub fn build<F>(behavior: F, n_workers: usize) -> std::io::Result<Self>
    where F: std::ops::Fn(Message) -> std::io::Result<()>,
          F: Send,
          F: Sync,
          F: 'static,
          F: Copy,
          Message: 'static {
        let consumer_handles: Vec<ConsumerHandle<Message>> = (1..n_workers).map(|x| {
            ConsumerHandle::build(behavior)
        }).collect::<std::io::Result<Vec<ConsumerHandle<Message>>>>()?;
        Ok(ConsumerGroup
        { consumer_handles
        , last_sent: 0
        }
        )
    }

    pub fn send(&mut self, message: Message) -> std::result::Result<(), std::sync::mpsc::SendError<Message>> {
        let n = self.consumer_handles.len();
        let consumer_handle = &mut self.consumer_handles[self.last_sent];
        self.last_sent = (self.last_sent + 1) % n;
        consumer_handle.send(message)
    }
}
