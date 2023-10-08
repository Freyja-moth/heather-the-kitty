use std::str::FromStr;

use crate::prelude::*;

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("ignored_channels")
        .description("Lists all ignored channels.")
        .default_member_permissions(Permissions::MANAGE_CHANNELS)
}

async fn try_run(guild_id: &GuildId, database: &MySqlPool) -> KittyResult<Vec<Option<ChannelId>>> {
    let channels: Vec<(String,)> = sqlx::query_as(SELECT_IGNORED)
        .bind(guild_id.to_string())
        .fetch_all(database)
        .await
        .map_err(DatabaseError::UnableToFindIgnoredChannels)?;

    if channels.is_empty() {
        Err(CommandError::NoChannelsFound.into())
    } else {
        Ok(channels
            .iter()
            .map(|channel| ChannelId::from_str(&channel.0).ok())
            .collect())
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

    match try_run(&guild_id, database).await {
        Ok(val) => {
            let message_response: String = val
                .into_iter()
                .enumerate()
                .map(|(num, channel)| {
                    if let Some(channel) = channel {
                        format!("{} - {}\n", num + 1, channel.mention())
                    } else {
                        format!("{} - Invalid\n", num + 1)
                    }
                })
                .collect();
            succeded_response(interaction, http, message_response).await?;
        }
        Err(KittyError::Command(CommandError::NoChannelsFound)) => {
            succeded_response(interaction, http, "No channels are ignored".to_string()).await?;
        }
        Err(err) => err.send_error_response(http, interaction).await?,
    }
    Ok(())
}
