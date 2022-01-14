use serenity::{ async_trait, model::{ channel::Message, gateway::Ready}, prelude::*};
use std::io::prelude::*;
use std::env;
use std::fs::OpenOptions;
use std::error::Error;



struct Handler;

#[async_trait]
impl EventHandler for Handler {

    async fn ready(&self, _: Context, ready: Ready) {
        // Log at the INFO level. This is a macro from the `tracing` crate.
        println!("{} is connected!", ready.user.name);
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.contains("!bug") {
            match write_to_file(&msg){
                Ok(m) =>{
                    let _ = m
                    .channel_id
                    .send_message(&ctx.http, |m| {
                        m.content("Hello, World!")
                    })
                    .await;

                },
                Err(e) =>{

                }
            }
        }
    }
}

//Self explanatory function
fn write_to_file(msg: &Message) -> Result<&Message, Box<dyn Error + Send + Sync>>{
    let path = format!("./{}.txt", msg.channel_id.to_string());
    let content = msg.content.replace("!bug ", "");
    let text =format!("From: {}\nDate: {}\nMessage:{}\n\n", msg.author.name, msg.timestamp.to_string(), content);
    let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(path)?;
    file.write_all(text.as_bytes())?;
    Ok(msg)
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
