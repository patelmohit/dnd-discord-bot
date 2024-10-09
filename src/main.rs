use std::env;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if let Err(why) = msg.channel_id.say(&ctx.http, "hi there!").await {
            println!("Error sending message: {:?}", why);
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let discord_token =
        env::var("DISCORD_TOKEN").expect("Expected DISCORD_TOKEN in the environment");
    let application_id = env::var("APP_ID")
        .expect("Expected APP_ID in the environment")
        .parse::<u64>()
        .expect("Failed to parse APP_ID to integer");

    let mut client = Client::builder(&discord_token)
        .event_handler(Handler)
        .application_id(application_id)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
