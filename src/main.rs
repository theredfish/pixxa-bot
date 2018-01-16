extern crate pixxa;
extern crate serenity;

extern crate dotenv;
#[macro_use]
extern crate dotenv_codegen;

use pixxa::technobabble::Technobabble;
use serenity::prelude::*;
use serenity::model::*;

struct Handler;

impl EventHandler for Handler {
    fn on_message(&self, _: Context, msg: Message) {
        if msg.content == "!technobabble" {
            let pixxa_babble = match Technobabble::new() {
                Ok(technobabble) => technobabble.generate(),
                Err(e) => String::from(e)
            };

            if let Err(why) = msg.channel_id.say(pixxa_babble) {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    fn on_ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

fn main() {
    dotenv::dotenv().ok();
    let token = dotenv!("DISCORD_TOKEN");
    
    let mut client = Client::new(&token, Handler);

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
