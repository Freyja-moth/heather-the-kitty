use std::borrow::Cow;

use serenity::{
    builder::CreateApplicationCommand,
    model::{
        prelude::{
            command::CommandOptionType,
            interaction::application_command::{
                ApplicationCommandInteraction, CommandDataOptionValue,
            },
            PartialChannel,
        },
        Permissions,
    },
    prelude::Context,
};
use sqlx::Error as SqlxError;

use crate::events::get_database;

use super::response::Response;

#[derive(Debug)]
pub enum FocusError {
    CannotAddToDatabase,
    CannotFindChannel,
    ChannelAlreadyIgnored,
}
impl ToString for FocusError {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}
impl From<SqlxError> for FocusError {
    fn from(value: SqlxError) -> Self {
        match value {
            SqlxError::Database(database) if database.code() == Some(Cow::from("23000")) => {
                Self::ChannelAlreadyIgnored
            }
            _ => Self::CannotAddToDatabase,
        }
    }
}

pub struct RemovedChannel(PartialChannel);

impl ToString for RemovedChannel {
    fn to_string(&self) -> String {
        self.0.name.clone().unwrap()
    }
}

type FocusResult<T> = Result<T, FocusError>;

async fn add_to_db(ctx: &Context, channel: &PartialChannel) -> FocusResult<()> {
    let pool = get_database(ctx).await;

    let channel_id = channel.id.to_string();

    sqlx::query!(
        "DELETE FROM ignore_channels WHERE channel_id = ?",
        channel_id
    )
    .execute(&*pool)
    .await?;

    Ok(())
}

pub async fn run(command: ApplicationCommandInteraction, ctx: &Context) {
    async {
        let channel = command
            .data
            .options
            .first()
            // There should only be one value here, so we can ignore the others
            .ok_or(FocusError::CannotFindChannel)
            .and_then(|data| {
                // Checks that
                // 1. There is a CommandDataOptionValue
                // 2. That the value in it is actually a channel
                if let Some(CommandDataOptionValue::Channel(channel)) = data.resolved.clone() {
                    Ok(channel)
                } else {
                    Err(FocusError::CannotFindChannel)
                }
            })?;

        add_to_db(ctx, &channel).await?;

        Ok::<RemovedChannel, FocusError>(RemovedChannel(channel))
    }
    .await
    // Pass the result of the above closure? (I think it might be called something else) and create
    // a response using it
    .create_response(ctx, command)
    .await;
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("focus")
        .description("Unignores a specific channel")
        .create_option(|option| {
            option
                .name("channel")
                .description("The channel to be unignored")
                .kind(CommandOptionType::Channel)
                .required(true)
        })
        .default_member_permissions(Permissions::MANAGE_CHANNELS)
}
