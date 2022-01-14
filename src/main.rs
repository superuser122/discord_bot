use serenity::{ async_trait, model::{ channel::Message, gateway::Ready}, prelude::*};
use std::io::prelude::*;
use std::env;
use std::fs::OpenOptions;



struct Handler;

#[async_trait]
impl EventHandler for Handler {

    async fn ready(&self, _: Context, ready: Ready) {
        // Log at the INFO level. This is a macro from the `tracing` crate.
        println!("{} is connected!", ready.user.name);
    }

    async fn message(&self, _ctx: Context, msg: Message) {
        if msg.content.contains("!bug") {
            write_to_file(msg);
        }
    }
}

//Self explanatory function
fn write_to_file(msg: Message){
    let path = format!("./{}.txt", msg.channel_id.to_string());
    let content = msg.content.replace("!bug ", "");
    let text =format!("Από: {}\nΗμ/νία: {}\nΜήνυμα:{}\n\n", msg.author.name, msg.timestamp.to_string(), content);
    let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(path).unwrap();
    file.write_all(text.as_bytes()).unwrap();
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let mut client =
        Client::builder(&token).event_handler(Handler).await.expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
