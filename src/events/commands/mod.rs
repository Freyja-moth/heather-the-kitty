use std::sync::Arc;

use serenity::{
    model::{
        application::interaction::application_command::ApplicationCommandInteraction,
        prelude::interaction::InteractionResponseType,
    },
    prelude::Context,
};
use sqlx::MySqlPool;

use crate::Database;

pub mod ignore;

async fn get_database(ctx: &Context) -> CommandResult<Arc<MySqlPool>> {
    let reader = ctx.data.read().await;
    Ok(reader
        .get::<Database>()
        .ok_or(CommandsError::CannotFetchDatabase)?
        .clone())
}

pub async fn handle_commands(ctx: Context, command: ApplicationCommandInteraction) {
    match command.data.name.as_str() {
        "ignore" => {}
        _ => {}
    }
}

#[derive(Debug)]
enum CommandsError {
    CannotFetchDatabase,
    CannotUpdateDatabase,
    CannotFindOptions,
    CannotFindChannel,
}
impl ToString for CommandsError {
    fn to_string(&self) -> String {
        format!("{self:?}")
    }
}
impl CommandsError {
    fn create_response(self, ctx: &Context, command: ApplicationCommandInteraction) {
        command.create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|content| {
                    content.embed(|embed| {
                        embed.title("Error!").description(format!(
                            "Command has failed\nReason: {}",
                            self.to_string()
                        ))
                    })
                })
        });
    }
}
type CommandResult<T> = Result<T, CommandsError>;
