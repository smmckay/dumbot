use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::framework::Framework;
use threadpool::ThreadPool;

pub mod dadjoke;
pub mod im;

pub trait MessageHandler {
    fn message(&self, ctx: &Context, msg: &Message, pool: &ThreadPool);
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
        for child in self.children.iter() {
            child.message(&ctx, &msg, pool)
        }
    }
}
