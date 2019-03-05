#[macro_use] extern crate log;
extern crate regex;

extern crate log4rs;
extern crate serenity;

use std::env;
use regex::Regex;

struct Handler {
    im_re: Regex
}

impl serenity::client::EventHandler for Handler {
    fn message(&self, _: serenity::client::Context, msg: serenity::model::channel::Message) {
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

    fn ready(&self, _: serenity::client::Context, ready: serenity::model::gateway::Ready) {
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

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client = match serenity::client::Client::new(&token, Handler::new())  {
        Err(why) => panic!("Client error: {:?}", why),
        Ok(client) => client
    };

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
