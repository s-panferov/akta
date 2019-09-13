mod actor;
mod addr;
mod context;
mod kernel;
mod message;

pub use self::actor::{Actor, Output};
pub use self::addr::Addr;
pub use self::context::Context;
pub use self::kernel::{start_actor, start_actor_unbounded};
pub use self::message::{Ask, Message, WithReply};

pub mod threadpool {
    pub use tokio_executor::threadpool::{Builder, ThreadPool};
}

pub mod prelude {
    pub use super::*;
    pub use futures::sink::SinkExt;
}
