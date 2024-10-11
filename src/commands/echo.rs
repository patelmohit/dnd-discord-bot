use serenity::all::{
    CommandOptionType, CreateCommand, CreateCommandOption, ResolvedOption, ResolvedValue,
};

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
