use std::borrow::Cow;

use serenity::{
    builder::CreateApplicationCommand,
    model::{
        prelude::{
            application::interaction::application_command::ApplicationCommandInteraction,
            command::CommandOptionType, interaction::application_command::CommandDataOptionValue,
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
pub enum IgnoreError {
    CannotAddToDatabase,
    CannotFindChannel,
    ChannelAlreadyIgnored,
}
impl ToString for IgnoreError {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}
impl From<SqlxError> for IgnoreError {
    fn from(value: SqlxError) -> Self {
        match value {
            SqlxError::Database(database) if database.code() == Some(Cow::from("23000")) => {
                Self::ChannelAlreadyIgnored
            }
            _ => Self::CannotAddToDatabase,
        }
    }
}

pub struct AddedChannel(PartialChannel);

impl ToString for AddedChannel {
    fn to_string(&self) -> String {
        self.0.name.clone().unwrap()
    }
}

type IgnoreResult<T> = Result<T, IgnoreError>;

async fn add_to_db(ctx: &Context, channel: &PartialChannel) -> IgnoreResult<()> {
    let pool = get_database(ctx).await;

    let channel_id = channel.id.to_string();

    sqlx::query!(
        "INSERT INTO ignore_channels(channel_id) VALUES(?)",
        channel_id
    )
    .execute(&*pool)
    .await?;

    Ok(())
}

pub async fn run(command: ApplicationCommandInteraction, ctx: &Context) {
    let response: IgnoreResult<AddedChannel> = async {
        let channel = command
            .data
            .options
            .first()
            .ok_or(IgnoreError::CannotFindChannel)
            .and_then(|data| {
                if let Some(CommandDataOptionValue::Channel(channel)) = data.resolved.clone() {
                    Ok(channel)
                } else {
                    Err(IgnoreError::CannotFindChannel)
                }
            })?;

        add_to_db(ctx, &channel).await?;

        Ok(AddedChannel(channel))
    }
    .await;

    response.create_response(ctx, command).await;
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("ignore")
        .description("Ignores a specific channel")
        .create_option(|option| {
            option
                .name("channel")
                .description("The channel to be ignored")
                .kind(CommandOptionType::Channel)
                .required(true)
        })
        .default_member_permissions(Permissions::MANAGE_CHANNELS)
}
