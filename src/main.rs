extern crate chashmap;
#[macro_use] extern crate log;
extern crate log4rs;
extern crate rand;
extern crate regex;
extern crate serenity;

use std::env;
use serenity::Client;
use serenity::prelude::EventHandler;
use serenity::client::Context;
use serenity::model::gateway::Ready;

mod handlers;

fn init_logging() {
    let console_appender = log4rs::append::console::ConsoleAppender::builder()
        .encoder(Box::new(log4rs::encode::json::JsonEncoder::new()))
        .build();

    let cb = log4rs::config::Config::builder()
        .appender(log4rs::config::Appender::builder().build("stderr", Box::new(console_appender)))
        .logger(log4rs::config::Logger::builder().build("dumbot", log::LevelFilter::Debug));

    let rb = log4rs::config::Root::builder()
        .appender("stderr");

    let config = cb.build(rb.build(log::LevelFilter::Info)).expect("Failed building config");
    log4rs::init_config(config).expect("Failed logger init");
}

struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}

fn main() {
    init_logging();

    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment variable DISCORD_TOKEN");

    let im_cooldown_secs: u64 = env::var("IM_COOLDOWN_SECS")
        .unwrap_or("3600".to_owned())
        .parse().unwrap();
    let im_chance_percent: u32 = env::var("IM_CHANCE_PERCENT")
        .unwrap_or("25".to_owned())
        .parse().unwrap();
    let im_handler = handlers::im::Handler::new(im_cooldown_secs, im_chance_percent);
    let dadjoke_handler = handlers::dadjoke::Handler::new();
    let handler_chain = handlers::Chain::new(vec![
        Box::new(im_handler),
        Box::new(dadjoke_handler)
    ]);

    let mut client = match Client::new(&token, Handler)  {
        Err(why) => panic!("Client error: {:?}", why),
        Ok(client) => client
    };
    client.with_framework(handler_chain);

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
