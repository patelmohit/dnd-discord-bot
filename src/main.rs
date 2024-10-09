use std::env;

use poise::serenity_prelude::{self as serenity, ClientBuilder, GatewayIntents, GuildId};

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Responds with "world!"
#[poise::command(slash_command)]
async fn hello(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("world!").await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let discord_token =
        env::var("DISCORD_TOKEN").expect("Expected DISCORD_TOKEN in the environment");
    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::GUILD_MESSAGES;
    let guild_id = GuildId::new(
        env::var("GUILD_ID")
            .expect("Expected GUILD_ID in environment")
            .parse()
            .expect("GUILD_ID must be an integer"),
    );

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![hello()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = ClientBuilder::new(discord_token, GatewayIntents::non_privileged())
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap();
}
