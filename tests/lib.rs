use std::sync::Arc;

use akta::prelude::*;
use akta::threadpool::Builder;
use futures::channel::mpsc::UnboundedSender;

struct Echo {}

impl Actor for Echo {
	type Message = WithReply<u32, u32>;
	type Sender = UnboundedSender<Self::Message>;

	fn handle(&mut self, msg: Self::Message, _ctx: &Context<Self>) -> Output {
		Box::pin(async {
			let (msg, answer) = msg.into_parts();
			let _ = answer.send(msg);
		})
	}
}

#[test]
fn test() {
	futures::executor::block_on(async {
		let ex = Arc::new(Builder::new().build());
		let mut addr = start_actor_unbounded(ex, Echo {});
		let res: u32 = addr.ask(WithReply::new(10)).await.unwrap();

		assert_eq!(res, 10)
	});
}
