use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::framework::Framework;
use threadpool::ThreadPool;

pub mod dadjoke;
pub mod im;

pub trait MessageHandler {
    fn message(&self, ctx: &Context, msg: &Message, pool: &ThreadPool) -> bool;
}

pub struct Chain {
    children: Vec<Box<MessageHandler + Send + Sync>>
}

impl Chain {
    pub fn new(children: Vec<Box<MessageHandler + Send + Sync>>) -> Chain {
        Chain {
            children
        }
    }
}

impl Framework for Chain {
    fn dispatch(&mut self, ctx: Context, msg: Message, pool: &ThreadPool) {
        debug!("{}", msg.content.as_str());
        for child in self.children.iter() {
            if child.message(&ctx, &msg, pool) {
                return
            }
        }
    }
}
