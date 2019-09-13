use futures::channel::oneshot::{Canceled, Receiver as OneshotReceiver};
use futures::sink::SinkExt;

use crate::actor::Actor;
use crate::message::Ask;

#[derive(Clone)]
pub struct Addr<A: Actor> {
	sender: A::Sender,
}

impl<A: Actor> Addr<A> {
	pub fn new(sender: A::Sender) -> Self {
		Addr { sender }
	}

	pub async fn send(
		&mut self,
		msg: <A as Actor>::Message,
	) -> Result<
		(),
		<<A as Actor>::Sender as futures::sink::Sink<
			<A as Actor>::Message,
		>>::Error,
>{
		self.sender.send(msg).await
	}

	pub async fn ask(
		&mut self,
		msg: (
			<A as Actor>::Message,
			OneshotReceiver<<<A as Actor>::Message as Ask>::Response>,
		),
	) -> Result<<<A as Actor>::Message as Ask>::Response, Canceled>
	where
		<A as Actor>::Message: Ask,
	{
		let res = self.sender.send(msg.0).await;
		match res {
			Ok(_) => msg.1.await,
			Err(_) => Err(Canceled),
		}
	}
}
