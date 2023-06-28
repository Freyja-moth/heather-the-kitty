use log::{error, info};
use serenity::{
    builder::{CreateApplicationCommand, CreateInteractionResponse},
    model::prelude::{
        application::interaction::application_command::ApplicationCommandInteraction,
        command::CommandOptionType,
        interaction::{
            application_command::{CommandDataOption, CommandDataOptionValue},
            InteractionResponseType,
        },
        PartialChannel,
    },
    prelude::Context,
};

use crate::{
    error::{CatError, CatResult},
    Database,
};

use super::{get_database, CommandResult, CommandsError};

async fn add_to_db(ctx: &Context, channel: &CommandDataOption) -> CommandResult<()> {
    let pool = get_database(ctx).await?;

    let CommandDataOptionValue::Channel(channel) =
        channel.resolved.clone().ok_or(CommandsError::CannotFindChannel)? else {
            return Err(CommandsError::CannotFindChannel);
        };
    let channel_id = channel.id.to_string();

    sqlx::query!(
        "INSERT INTO ignore_channels(channel_id) VALUES(?)",
        channel_id
    )
    .execute(&*pool)
    .await
    .map_err(|_| CommandsError::CannotUpdateDatabase)?;

    Ok(())
}

pub async fn run(command: ApplicationCommandInteraction, ctx: &Context) {
    let response = async {
        let channel = command
            .data
            .options
            .first()
            .ok_or(CommandsError::CannotFindOptions)?;
        add_to_db(ctx, channel).await?;

        Ok::<(), CommandsError>(())
    };
}

// pub async fn run<'a>(command: ApplicationCommandInteraction, ctx: &Context) {
// let Some(channel) = command.data.options.first().and_then(|option| option.resolved.as_ref().and_then(|data_option| {
// if let CommandDataOptionValue::Channel(channel) = data_option {
// Some(channel)
// } else {
// None
// }
// })) else {
// command.create_interaction_response(ctx.http, |response| {
// response
// .kind(InteractionResponseType::ChannelMessageWithSource)
// .interaction_response_data(|message| {
// message.content("Sorry, there was an issue when finding that channel. Please try again in a bit")
// }) });
//
// command.create_interaction_response(ctx.http, |response| {

// });
// return;
// };

// return response
// .kind(InteractionResponseType::ChannelMessageWithSource)
// .interaction_response_data(|message| {
// message.content(format!(
// "Added channel {} to ignore list!",
// channel
// .clone()
// .name
// .unwrap_or("<channel name not found>".to_string())
// ))
// });

// add_to_db(ctx, channel).await;
// }

pub fn register<'a>(command: &'a mut CreateApplicationCommand) -> &'a mut CreateApplicationCommand {
    command
        .name("ignore")
        .description("Ignore a specific channel")
        .create_option(|option| {
            option
                .name("channel")
                .description("The channel to be ignored")
                .kind(CommandOptionType::Channel)
                .required(true)
        })
}
