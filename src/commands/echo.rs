use serenity::all::CreateCommandOption;
use serenity::all::ResolvedValue;
use serenity::builder::CreateCommand;
use serenity::model::application::CommandOptionType;
use serenity::model::application::ResolvedOption;

pub fn run(options: &[ResolvedOption]) -> String {
    if let Some(ResolvedOption {
        value: ResolvedValue::String(val),
        ..
    }) = options.first()
    {
        format!("{}", val)
    } else {
        "Please provide a valid string".to_string()
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("echo")
        .description("Returns whatever you say!")
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "value", "A value to echo")
                .required(true),
        )
}
