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
            let response = match write_to_file(&msg){
                Ok(_) => String::from("Bug report was saved"),
                Err(e) => format!("Error saving bug report: {}", e.to_string())
            };

            let r = msg.channel_id.send_message(&ctx.http, |m| {
                m.content(response)}).await;
            if let Err(why) = r {
                println!("Bot response error: {:?}", why);
            }
        }
    }
}

//Self explanatory function
fn write_to_file(msg: &Message) -> Result<(), Box<dyn Error + Send + Sync>>{
    let path = format!("./{}.txt", msg.channel_id.to_string());
    let content = msg.content.replace("!bug ", "");
    let text =format!("From: {}\nDate: {}\nMessage:{}\n\n", msg.author.name, msg.timestamp.to_string(), content);
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(path)?;
    file.write_all(text.as_bytes())?;
    Ok(())
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    let mut client =
        Client::builder(token, intents).event_handler(Handler).await.expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
