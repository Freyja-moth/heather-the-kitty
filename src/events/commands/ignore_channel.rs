use crate::prelude::*;

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("ignore_channel")
        .description("Marks the channel to be ignored. Heather won't react to the channel but commands can still be used.")
        .default_member_permissions(Permissions::MANAGE_CHANNELS)
        .create_option(|option|
            option.name("channel")
                .description("The channel to be ignored")
                .kind(CommandOptionType::Channel).required(true)
        )
}

async fn try_run<'a>(
    options: &'a [CommandDataOption],
    guild_id: GuildId,
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

    if sqlx::query(INSERT_CHANNEL)
        .bind(channel.id.to_string())
        .bind(guild_id.0.to_string())
        .execute(database)
        .await
        .map_err(DatabaseError::UnableToInsertChannelToTable)?
        .rows_affected()
        == 0
    {
        Err(CommandError::ChannelIsAlreadyIgnored.into())
    } else {
        info!(
            "Added channel with id: {} to ignore list",
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
    let guild_id = interaction
        .guild_id
        .ok_or(CommandError::CommandRanOutsideOfGuild)?;
    let options = &interaction.data.options;

    match try_run(options, guild_id, database).await {
        Ok(channel) => {
            succeded_response(
                interaction,
                http,
                format!("I've added channel {} to ignore list", channel.id.mention()),
            )
            .await?
        }
        Err(err) => err.send_error_response(http, interaction).await?,
    }
    Ok(())
}
