use log::error;
use serenity::{
    async_trait,
    builder::CreateApplicationCommands,
    model::prelude::{interaction::Interaction, GuildId, Ready},
    prelude::{Context, EventHandler},
};

use self::commands::handle_commands;

mod commands;

pub struct Events;

#[async_trait]
impl EventHandler for Events {
    async fn ready(&self, ctx: Context, _ready: Ready) {
        let guild = GuildId(985827699853492274);

        guild
            .set_application_commands(&ctx.http, setup_commands)
            .await
            .and_then(|commands| {
                commands
                    .into_iter()
                    .map(|command| command.name)
                    .for_each(|name| println!("Created command: {name}"));
                Ok(())
            })
            .unwrap_or_else(|err| error!("Cannot create application commands: {err}"));
    }
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            handle_commands(ctx, command).await;
        }
    }
}

fn setup_commands<'a>(
    commands: &'a mut CreateApplicationCommands,
) -> &'a mut CreateApplicationCommands {
    commands.create_application_command(commands::ignore::register)
}
