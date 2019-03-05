#[macro_use] extern crate log;
extern crate log4rs;
extern crate regex;
extern crate serenity;

use regex::Regex;
use serenity::client::EventHandler;
use serenity::model::{gateway::Ready, channel::Message};
use serenity::prelude::{Context, Client};
use std::env;

struct Handler {
    im_re: Regex
}

impl EventHandler for Handler {
    fn message(&self, _: Context, msg: Message) {
        if let Some(captures) = self.im_re.captures(msg.content.as_str()) {
            let new_nick = captures.get(1).unwrap().as_str();
            msg.guild_id.map(|gid|
                if let Err(why) = gid.edit_member(msg.author.id, |m| m.nickname(new_nick)) {
                    warn!("Unable to update {}: {:?}", msg.author.id, why)
                }
            );

            if let Err(why) = msg.channel_id.send_message(|m| m.content(format!("Hi {}, I'm Dumbot!", new_nick))) {
                warn!("Msg send failed: {:?}", why)
            }
        }
    }

    fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}

impl Handler {
    fn new() -> Handler {
        Handler {
            im_re: Regex::new("^\\s*(?i:i\\pPm|i\\s+am|im)\\s+([^.]+)").unwrap()
        }
    }
}

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

fn main() {
    init_logging();

    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment variable DISCORD_TOKEN");

    let mut client = match Client::new(&token, Handler::new())  {
        Err(why) => panic!("Client error: {:?}", why),
        Ok(client) => client
    };

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
