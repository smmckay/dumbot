use chashmap::CHashMap;
use rand::Rng;
use regex::Regex;
use serenity::client::Context;
use serenity::model::channel::Message;
use std::time::Instant;
use crate::handlers::MessageHandler;
use threadpool::ThreadPool;

pub struct Handler {
    im_cooldown_secs: u64,
    im_chance_percent: u32,
    im_re: Regex,
    im_last_activity: CHashMap<u64, Instant>,
    start: std::time::Instant
}

impl MessageHandler for Handler {
    fn message(&self, _: &Context, msg: &Message, _: &ThreadPool) -> bool {
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
            true
        } else {
            false
        }
    }
}

impl Handler {
    pub fn new(im_cooldown_secs: u64, im_chance_percent: u32) -> Handler {
        Handler {
            im_cooldown_secs,
            im_chance_percent,
            im_re: Regex::new("^\\s*(?i:i\\pPm|i\\s+am|im)\\s+([^.]+)").unwrap(),
            im_last_activity: CHashMap::new(),
            start: Instant::now() - std::time::Duration::from_secs(im_cooldown_secs)
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

        if Instant::now().duration_since(last_nick_change).as_secs() < self.im_cooldown_secs {
            return None
        }

        self.im_re.captures(msg.content.as_str())
            .and_then(|captures| captures.get(1))
            .map(|m| {
                let s = m.as_str();
                if s.chars().count() <= 32 {
                    s
                } else {
                    let substr_end = s.char_indices().take(32).last().unwrap().0;
                    &s[..substr_end]
                }
            })
    }
}