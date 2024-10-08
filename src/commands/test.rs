use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

// https://github.com/serenity-rs/serenity/blob/current/examples/e14_slash_commands/src/commands/ping.rs

pub fn run(_options: &[ResolvedOption]) -> String {
    "meowing from rust".to_string()
}

pub fn register() -> CreateCommand {
    CreateCommand::new("test").description("I'll meow if I'm working!")
}
