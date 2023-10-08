pub mod focus_channel;
pub mod hug;
pub mod ignore_channel;
pub mod ignored_channels;
pub mod is_ignored;
pub mod pat;
pub mod scratch;

use crate::prelude::*;

pub async fn create_commands(http: &Arc<Http>) -> KittyResult {
    GuildId(985827699853492274)
        .set_application_commands(http, setup_commands)
        .await
        .map_err(CommandError::CannotCreateCommand)?
        .iter()
        .for_each(|command| info!("Created testing command, {}", command.name));
    Ok(())
}

fn setup_commands(commands: &mut CreateApplicationCommands) -> &mut CreateApplicationCommands {
    commands
        .create_application_command(ignore_channel::register)
        .create_application_command(focus_channel::register)
        .create_application_command(is_ignored::register)
        .create_application_command(ignored_channels::register)
        .create_application_command(hug::register)
        .create_application_command(pat::register)
        .create_application_command(scratch::register)
}

pub async fn run_commands(ctx: &Context, interaction: &Interaction) -> KittyResult {
    let database = Database::retrieve_database(ctx)
        .await
        .ok_or(DatabaseError::CannotRetrieveDatabase)?;

    if let Interaction::ApplicationCommand(command) = interaction {
        match command.data.name.as_str() {
            "ignore_channel" => ignore_channel::run(command, &ctx.http, database.inner()).await,
            "focus_channel" => focus_channel::run(command, &ctx.http, database.inner()).await,
            "is_ignored" => is_ignored::run(command, &ctx.http, database.inner()).await,
            "ignored_channels" => ignored_channels::run(command, &ctx.http, database.inner()).await,
            "hug" => hug::run(command, &ctx.http).await,
            "pat" => pat::run(command, &ctx.http).await,
            "scritches" => scratch::run(command, &ctx.http).await,
            _ => Err(CommandError::CannotFindCommand.into()),
        }?
    }
    Ok(())
}
