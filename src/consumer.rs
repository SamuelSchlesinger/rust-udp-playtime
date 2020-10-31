pub struct ConsumerHandle<Message, Environment> {
    sender: std::sync::mpsc::SyncSender<Message>,
    join_handle: std::thread::JoinHandle<Message>,
    phantom_environment: std::marker::PhantomData<Environment>,
}

impl<Message, Environment> ConsumerHandle<Message, Environment>
where
    Message: Send,
    Environment: Send,
{
    pub fn build(
        behavior: std::sync::Arc<
            dyn Send + Sync + std::ops::Fn(Message, &mut Environment) -> std::io::Result<()>,
        >,
        environment: Environment,
    ) -> std::io::Result<Self>
    where
        Environment: Clone,
        Environment: 'static,
        Message: 'static,
    {
        let (sender, receiver) = std::sync::mpsc::sync_channel(4096);
        let mut environment = environment.clone();
        let join_handle = std::thread::spawn(move || loop {
            if let Ok(message) = receiver.recv() {
                if let Err(err) = behavior(message, &mut environment) {
                    println!("Received error: {:?}", err);
                }
            }
        });
        Ok(ConsumerHandle {
            sender,
            join_handle,
            phantom_environment: std::marker::PhantomData,
        })
    }

    pub fn send(
        &mut self,
        message: Message,
    ) -> std::result::Result<(), std::sync::mpsc::SendError<Message>> {
        self.sender.send(message)
    }

    pub fn join_handle(self) -> std::thread::JoinHandle<Message> {
        self.join_handle
    }
}

pub struct ConsumerGroup<Message, Environment> {
    consumer_handles: Vec<ConsumerHandle<Message, Environment>>,
    last_sent: usize,
}

impl<Message, Environment> ConsumerGroup<Message, Environment>
where
    Message: Send,
    Environment: Send,
{
    pub fn build(
        behavior: std::sync::Arc<
            dyn Send + Sync + std::ops::Fn(Message, &mut Environment) -> std::io::Result<()>,
        >,
        environment: &Environment,
        n_workers: usize,
    ) -> std::io::Result<Self>
    where
        Environment: Clone,
        Message: 'static,
        Environment: 'static,
    {
        let consumer_handles: Vec<ConsumerHandle<Message, Environment>> = (0..n_workers)
            .map(|_| ConsumerHandle::build(behavior.clone(), environment.clone()))
            .collect::<std::io::Result<Vec<ConsumerHandle<Message, Environment>>>>()?;
        Ok(ConsumerGroup {
            consumer_handles,
            last_sent: 0,
        })
    }

    pub fn send(
        &mut self,
        message: Message,
    ) -> std::result::Result<(), std::sync::mpsc::SendError<Message>> {
        let n = self.consumer_handles.len();
        let consumer_handle = &mut self.consumer_handles[self.last_sent];
        self.last_sent = (self.last_sent + 1) % n;
        consumer_handle.send(message)
    }
}
