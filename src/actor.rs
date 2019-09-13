use std::pin::Pin;

use crate::context::Context;
use crate::message::Message;

use futures::sink::Sink;
use futures::Future;

pub type Output<'a> = Pin<Box<dyn Future<Output = ()> + Send + 'a>>;

pub trait Actor: Sized + Send + 'static {
    type Message: Message + Send + 'static;
    type Sender: Sink<Self::Message> + Send + Sync + Clone + std::marker::Unpin;

    fn handle<'a>(&'a mut self, msg: Self::Message, ctx: &Context<Self>) -> Output<'a>;
}
