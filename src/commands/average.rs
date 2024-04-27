use twilight_model::{
    application::interaction::application_command::{CommandData, CommandOptionValue},
    channel::message::MessageFlags,
    gateway::payload::incoming::InteractionCreate,
    http::interaction::InteractionResponseData,
};
use twilight_util::builder::InteractionResponseDataBuilder;

use crate::context::Context;

pub async fn interaction(
    cmd: &CommandData,
    _int: &InteractionCreate,
    _ctx: &Context,
) -> anyhow::Result<InteractionResponseData> {
    let args = if let CommandOptionValue::String(args) = &cmd.options[0].value {
        args
    } else {
        unreachable!()
    };
    let array: Vec<usize> = args
        .split(';')
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()?;

    let out = array.iter().sum::<usize>() / array.len();

    Ok(InteractionResponseDataBuilder::new()
        .content(format!("Result: {out}"))
        .flags(MessageFlags::EPHEMERAL)
        .build())
}
