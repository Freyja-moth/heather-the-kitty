use crate::prelude::*;

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("is_ignored")
        .description("Checks whether or not the channel is ignored.")
        .default_member_permissions(Permissions::MANAGE_CHANNELS)
        .create_option(|option| {
            option
                .name("channel")
                .description("The channel you want to check")
                .kind(CommandOptionType::Channel)
                .required(true)
        })
}

async fn try_run(options: &[CommandDataOption], database: &MySqlPool) -> KittyResult<bool> {
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

    match sqlx::query(IS_IGNORED)
        .bind(channel.id.0.to_string())
        .fetch_one(database)
        .await
    {
        Ok(_) => Ok(true),
        Err(sqlx::Error::RowNotFound) => Ok(false),
        Err(err) => Err(DatabaseError::UnableToCheckIfChannelIsIgnored(err).into()),
    }
}

pub async fn run(
    interaction: &ApplicationCommandInteraction,
    http: impl AsRef<Http>,
    database: &MySqlPool,
) -> KittyResult {
    let options = &interaction.data.options;
    match try_run(options, database).await {
        Err(err) => err.send_error_response(http, interaction).await?,
        Ok(value) => {
            succeded_response(
                interaction,
                http,
                format!(
                    "{} is {}",
                    interaction.channel_id.mention(),
                    if value { "ignored" } else { "focused" }
                ),
            )
            .await?
        }
    }
    Ok(())
}
