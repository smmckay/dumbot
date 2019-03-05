extern crate chashmap;
#[macro_use] extern crate log;
extern crate log4rs;
extern crate rand;
extern crate regex;
extern crate serenity;

use chashmap::CHashMap;
use rand::Rng;
use regex::Regex;
use serenity::client::EventHandler;
use serenity::model::{gateway::Ready, channel::Message};
use serenity::prelude::{Context, Client};
use std::env;
use std::time::Instant;

struct Handler {
    im_cooldown: u64,
    im_chance_percent: u32,
    im_re: Regex,
    im_last_activity: CHashMap<u64, Instant>,
    start: std::time::Instant
}

impl EventHandler for Handler {
    fn message(&self, _: Context, msg: Message) {
        debug!("{}", msg.content.as_str());

        if let Some(new_nick) = self.get_new_nick(&msg) {
            msg.guild_id.map(|gid|
                if let Err(why) = gid.edit_member(msg.author.id, |m| m.nickname(new_nick)) {
                    warn!("Unable to update {}: {:?}", msg.author.id, why)
                }
            );

            if let Err(why) = msg.channel_id.send_message(|m| m.content(format!("Hi {}, I'm Dumbot!", new_nick))) {
                warn!("Msg send failed: {:?}", why)
            }

            self.im_last_activity.insert(msg.guild_id.unwrap().0, Instant::now());
        }
    }

    fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}

impl Handler {
    fn new() -> Handler {
        Handler {
            im_cooldown: 3600,
            im_chance_percent: 25,
            im_re: Regex::new("^\\s*(?i:i\\pPm|i\\s+am|im)\\s+([^.]+)").unwrap(),
            im_last_activity: CHashMap::new(),
            start: Instant::now()
        }
    }

    fn get_new_nick<'a>(&self, msg: &'a Message) -> Option<&'a str> {
        if rand::thread_rng().gen_range(0, 100) >= self.im_chance_percent {
           return None
        }

        if msg.guild_id.is_none() {
            return None
        }

        let gid = msg.guild_id.unwrap().0;

        let last_nick_change = match self.im_last_activity.get(&gid) {
            Some(v) => v.clone(),
            None => self.start
        };

        if Instant::now().duration_since(last_nick_change).as_secs() < self.im_cooldown {
            return None
        }

        self.im_re.captures(msg.content.as_str())
            .and_then(|captures| captures.get(1))
            .map(|m| m.as_str())
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
