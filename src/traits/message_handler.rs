use crate::implementations::channel::Message;

pub trait MessageHandler {
    fn handle_message(msg: Message);
}
