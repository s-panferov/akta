use futures::channel::oneshot::{channel, Receiver, Sender};
pub trait Message: Send + Sized + 'static {}

impl<T: Send + Sized + 'static> Message for T {}

pub trait Ask: Message {
    type Response: Send + 'static;
}

pub trait CancellableMessage {
    fn is_canceled(&self) -> bool;
}

pub struct WithReply<M: Send + 'static, R: Send + 'static> {
    message: M,
    sender: Sender<R>,
}

impl<M: Send + 'static, R: Send + 'static> WithReply<M, R> {
    pub fn new(message: M) -> (Self, Receiver<R>) {
        let (sender, receiver) = channel();
        (WithReply { message, sender }, receiver)
    }

    pub fn into_parts(self) -> (M, Sender<R>) {
        return (self.message, self.sender);
    }
}

impl<T: Send + 'static, R: Send + 'static> Ask for WithReply<T, R> {
    type Response = R;
}
