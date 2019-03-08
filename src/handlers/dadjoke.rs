use crate::handlers::MessageHandler;
use rand::Rng;
use regex::Regex;
use serenity::client::Context;
use serenity::model::channel::Message;
use threadpool::ThreadPool;

pub struct Handler {
    re: Regex,
    jokes: Vec<String>
}

impl MessageHandler for Handler {
    fn message(&self, _: &Context, msg: &Message, _: &ThreadPool) {
        if self.re.is_match(msg.content.as_str()) {
            let idx = rand::thread_rng().gen_range(0, self.jokes.len());
            let text = self.jokes.get(idx).unwrap();
            msg.channel_id.send_message(|m| m.content(text));
        }
    }
}

impl Handler {
    pub fn new() -> Handler {
        Handler {
            re: Regex::new("dad\\s*jo(?:ke|ek)").unwrap(),
            jokes: vec![
                "Did you hear about the restaurant on the moon? Great food, no atmosphere.".to_owned(),
                "What do you call a fake noodle? An Impasta.".to_owned(),
                "How many apples grow on a tree? All of them.".to_owned(),
                "Want to hear a joke about paper? Nevermind it's tearable.".to_owned(),
                "I just watched a program about beavers. It was the best dam program I've ever seen.".to_owned(),
                "Why did the coffee file a police report? It got mugged.".to_owned(),
                "How does a penguin build it's house? Igloos it together.".to_owned(),
                "Dad, did you get a haircut? No I got them all cut.".to_owned(),
                "What do you call a Mexican who has lost his car? Carlos.".to_owned(),
                "Dad, can you put my shoes on? No, I don't think they'll fit me.".to_owned(),
                "Why did the scarecrow win an award? Because he was outstanding in his field.".to_owned(),
                "Why don't skeletons ever go trick or treating? Because they have no body to go with.".to_owned(),
                "Ill call you later. Don't call me later, call me Dad.".to_owned(),
                "What do you call an elephant that doesn't matter? An irrelephant".to_owned(),
                "Want to hear a joke about construction? I'm still working on it.".to_owned(),
                "What do you call cheese that isn't yours? Nacho Cheese.".to_owned(),
                "Why couldn't the bicycle stand up by itself? It was two tired.".to_owned(),
                "What did the grape do when he got stepped on? He let out a little wine.".to_owned(),
                "I wouldn't buy anything with velcro. It's a total rip-off.".to_owned(),
                "The shovel was a ground-breaking invention.".to_owned(),
                "Dad, can you put the cat out? I didn't know it was on fire.".to_owned(),
                "This graveyard looks overcrowded. People must be dying to get in there.".to_owned(),
                "Whenever the cashier at the grocery store asks my dad if he would like the milk in a bag he replies, \"No, just leave it in the carton!".to_owned(),
                "5/4 of people admit that they’re bad with fractions.".to_owned(),
                "Two goldfish are in a tank. One says to the other, \"do you know how to drive this thing?".to_owned(),
                "What do you call a man with a rubber toe? Roberto.".to_owned(),
                "What do you call a fat psychic? A four-chin teller.".to_owned(),
                "I would avoid the sushi if I was you. It’s a little fishy.".to_owned(),
                "To the man in the wheelchair that stole my camouflage jacket... You can hide but you can't run.".to_owned(),
                "The rotation of earth really makes my day.".to_owned(),
                "I thought about going on an all-almond diet. But that's just nuts".to_owned(),
                "What's brown and sticky? A stick.".to_owned(),
                "I’ve never gone to a gun range before. I decided to give it a shot!".to_owned(),
                "Why do you never see elephants hiding in trees? Because they're so good at it.".to_owned(),
                "Did you hear about the kidnapping at school? It's fine, he woke up.".to_owned(),
                "A furniture store keeps calling me. All I wanted was one night stand.".to_owned(),
                "I used to work in a shoe recycling shop. It was sole destroying.".to_owned(),
                "Did I tell you the time I fell in love during a backflip? I was heels over head.".to_owned(),
                "I don’t play soccer because I enjoy the sport. I’m just doing it for kicks.".to_owned(),
                "People don’t like having to bend over to get their drinks. We really need to raise the bar.".to_owned()
            ]
        }
    }
}