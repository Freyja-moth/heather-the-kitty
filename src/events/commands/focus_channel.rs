use crate::prelude::*;

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("focus_channel")
        .description("Marks the channel to be focused. Heather will not react to messages sent.")
        .default_member_permissions(Permissions::MANAGE_CHANNELS)
        .create_option(|option| {
            option
                .name("channel")
                .description("The channel to be focused")
                .kind(CommandOptionType::Channel)
                .required(true)
        })
}

async fn try_run<'a>(
    options: &'a [CommandDataOption],
    database: &MySqlPool,
) -> KittyResult<&'a PartialChannel> {
    let channel = options
        .get(0)
        .ok_or(CommandError::CannotFindCommandOption)
        .and_then(|option| {
            option
                .resolved
                .as_ref()
                .ok_or(CommandError::CannotRetrieveOptionValue)
        })
        .and_then(|option| {
            if let CommandDataOptionValue::Channel(channel) = option {
                Ok(channel)
            } else {
                Err(CommandError::OptionValueIsOfWrongType)
            }
        })?;

    if sqlx::query(REMOVE_CHANNEL)
        .bind(channel.id.to_string())
        .execute(database)
        .await
        .map_err(DatabaseError::UnableToInsertChannelToTable)?
        .rows_affected()
        == 0
    {
        Err(CommandError::ChannelIsAlreadyFocused.into())
    } else {
        info!(
            "Removed channel with id: {} from ignore list",
            channel.id.to_string()
        );
        Ok(channel)
    }
}

pub async fn run(
    interaction: &ApplicationCommandInteraction,
    http: impl AsRef<Http>,
    database: &MySqlPool,
) -> KittyResult {
    let options = &interaction.data.options;

    match try_run(options, database).await {
        Ok(channel) => {
            succeded_response(
                interaction,
                http,
                format!(
                    "I've removed the channel {} from the ignore list!",
                    channel.id.mention()
                ),
            )
            .await?
        }
        Err(err) => err.send_error_response(http, interaction).await?,
    }

    Ok(())
}
