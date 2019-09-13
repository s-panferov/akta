use std::sync::Arc;

use tokio_executor::{blocking::run, threadpool::ThreadPool};

use crate::actor::Actor;
use crate::addr::Addr;

#[derive(Clone)]
pub struct Context<A: Actor> {
	ex: Arc<ThreadPool>,
	sender: A::Sender,
}

impl<A: Actor> Context<A> {
	pub fn new(ex: Arc<ThreadPool>, sender: A::Sender) -> Self {
		Context { ex, sender }
	}

	pub fn addr(&self) -> Addr<A> {
		Addr::new(self.sender.clone())
	}

	pub async fn blocking<F, T>(&self, fun: F) -> T
	where
		F: FnOnce() -> T + Send + 'static,
		T: Send + 'static,
	{
		run(fun).await
	}
}
