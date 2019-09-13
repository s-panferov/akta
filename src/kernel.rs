use std::sync::Arc;

use futures::channel::mpsc::{channel, unbounded, Sender, UnboundedSender};
use futures::stream::{Stream, StreamExt};
use tokio_executor::threadpool::ThreadPool;

use crate::actor::Actor;
use crate::addr::Addr;
use crate::context::Context;

pub fn start_kernel<A: Actor, R>(
	ex: Arc<ThreadPool>,
	mut actor: A,
	sender: A::Sender,
	mut receiver: R,
) -> Addr<A>
where
	R: Stream<Item = A::Message> + Send + Sync + std::marker::Unpin + 'static,
{
	let addr = sender.clone();
	let ex2 = ex.clone();

	ex.spawn(async move {
		let mut ctx = Context::new(ex2, sender);
		loop {
			let msg = receiver.next().await;
			match msg {
				Some(msg) => actor.handle(msg, &mut ctx).await,
				None => return,
			}
		}
	});

	Addr::new(addr)
}

pub fn start_actor<A: Actor>(
	ex: Arc<ThreadPool>,
	actor: A,
	queue: usize,
) -> Addr<A>
where
	Sender<A::Message>: Is<Type = <A as Actor>::Sender>,
{
	let (sender, receiver) = channel(queue);
	start_kernel(ex, actor, Is::into(sender), receiver)
}

pub fn start_actor_unbounded<A: Actor>(ex: Arc<ThreadPool>, actor: A) -> Addr<A>
where
	UnboundedSender<A::Message>: Is<Type = <A as Actor>::Sender>,
{
	let (sender, receiver) = unbounded();
	start_kernel(ex, actor, Is::into(sender), receiver)
}

pub trait Is {
	type Type;
	fn into(self) -> Self::Type;
}

impl<T> Is for T {
	type Type = T;
	fn into(self) -> Self::Type {
		self
	}
}
