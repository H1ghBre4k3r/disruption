use crate::channel::Message;

pub trait MessageCallback: Sized {
    fn handle_message(&self, msg: Message);
}

impl<F> MessageCallback for F
where
    F: Fn(Message),
{
    fn handle_message(&self, msg: Message) {
        self(msg);
    }
}
