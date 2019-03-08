use crate::handlers::MessageHandler;
use rand::Rng;
use regex::Regex;
use serenity::client::Context;
use serenity::model::channel::Message;
use std::thread;
use std::time::Duration;
use threadpool::ThreadPool;
use serenity::model::id::ChannelId;
use serenity::prelude::SerenityError;

pub struct Handler {
    re: Regex,
    jokes: Vec<Joke>
}

#[derive(Clone)]
struct Joke {
    setup: String,
    punchline: String
}

impl Joke {
    fn new(setup: &str, punchline: &str) -> Joke {
        Joke {
            setup: setup.to_owned(),
            punchline: punchline.to_owned()
        }
    }
}

impl MessageHandler for Handler {
    fn message(&self, _: &Context, msg: &Message, pool: &ThreadPool) -> bool {
        if self.re.is_match(msg.content.as_str()) {
            let channel = msg.channel_id;
            let idx = rand::thread_rng().gen_range(0, self.jokes.len());
            let joke = self.jokes.get(idx).unwrap().clone();

            pool.execute(move || {
                if let Err(why) = (|| -> Result<(), SerenityError> {
                    let delay: u64 = rand::thread_rng().gen_range(500, 1500);
                    send_msg_after_delay(channel, delay, &joke.setup)?;
                    if joke.punchline.len() > 0 {
                        send_msg_after_delay(channel, delay, &joke.punchline)?;
                    }
                    Ok(())
                })() {
                    warn!("API call failed: {:?}", why);
                }
            });

            true
        } else {
            false
        }
    }
}

impl Handler {
    pub fn new() -> Handler {
        Handler {
            re: Regex::new("(?i:dad\\s*jo(?:ke|ek))").unwrap(),
            jokes: vec![
                Joke::new("Did you hear about the restaurant on the moon?", "Great food, no atmosphere."),
                Joke::new("What do you call a fake noodle?", "An Impasta."),
                Joke::new("How many apples grow on a tree?", "All of them."),
                Joke::new("Want to hear a joke about paper?", "Nevermind it's tearable."),
                Joke::new("I just watched a program about beavers.", "It was the best dam program I've ever seen."),
                Joke::new("Why did the coffee file a police report?", "It got mugged."),
                Joke::new("How does a penguin build it's house?", "Igloos it together."),
                Joke::new("Dad, did you get a haircut?", "No I got them all cut."),
                Joke::new("What do you call a Mexican who has lost his car?", "Carlos."),
                Joke::new("Dad, can you put my shoes on?", "No, I don't think they'll fit me."),
                Joke::new("Why did the scarecrow win an award?", "Because he was outstanding in his field."),
                Joke::new("Why don't skeletons ever go trick or treating?", "Because they have no body to go with."),
                Joke::new("Ill call you later.", "Don't call me later, call me Dad."),
                Joke::new("What do you call an elephant that doesn't matter?", "An irrelephant"),
                Joke::new("Want to hear a joke about construction?", "I'm still working on it."),
                Joke::new("What do you call cheese that isn't yours?", "Nacho Cheese."),
                Joke::new("Why couldn't the bicycle stand up by itself?", "It was two tired."),
                Joke::new("What did the grape do when he got stepped on?", "He let out a little wine."),
                Joke::new("I wouldn't buy anything with velcro.", "It's a total rip-off."),
                Joke::new("The shovel was a ground-breaking invention.", ""),
                Joke::new("Dad, can you put the cat out?", "I didn't know it was on fire."),
                Joke::new("This graveyard looks overcrowded.", "People must be dying to get in there."),
                Joke::new("Whenever the cashier at the grocery store asks my dad if he would like the milk in a bag he replies, \"No, just leave it in the carton!\"", ""),
                Joke::new("5/4 of people admit that they’re bad with fractions.", ""),
                Joke::new("Two goldfish are in a tank. One says to the other, \"Do you know how to drive this thing?\"", ""),
                Joke::new("What do you call a man with a rubber toe?", "Roberto."),
                Joke::new("What do you call a fat psychic?", "A four-chin teller."),
                Joke::new("I would avoid the sushi if I was you.", "It’s a little fishy."),
                Joke::new("To the man in the wheelchair that stole my camouflage jacket...", "You can hide but you can't run."),
                Joke::new("The rotation of earth really makes my day.", ""),
                Joke::new("I thought about going on an all-almond diet.", "But that's just nuts"),
                Joke::new("What's brown and sticky?", "A stick."),
                Joke::new("I’ve never gone to a gun range before.", "I decided to give it a shot!"),
                Joke::new("Why do you never see elephants hiding in trees?", "Because they're so good at it."),
                Joke::new("Did you hear about the kidnapping at school?", "It's fine, he woke up."),
                Joke::new("A furniture store keeps calling me.", "All I wanted was one night stand."),
                Joke::new("I used to work in a shoe recycling shop.", "It was sole destroying."),
                Joke::new("Did I tell you the time I fell in love during a backflip?", "I was heels over head."),
                Joke::new("I don’t play soccer because I enjoy the sport.", "I’m just doing it for kicks."),
                Joke::new("People don’t like having to bend over to get their drinks.", "We really need to raise the bar.")
            ]
        }
    }
}

fn send_msg_after_delay(channel: ChannelId, millis: u64, text: &String) -> Result<(), SerenityError> {
    channel.broadcast_typing()?;
    thread::sleep(Duration::from_millis(millis));
    channel.send_message(|m| m.content(text))?;
    Ok(())
}
