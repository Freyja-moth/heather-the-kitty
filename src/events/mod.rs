mod commands;
mod reactions;

use std::sync::Arc;

use log::{error, info};
use serenity::{
    async_trait,
    builder::CreateApplicationCommands,
    model::{
        prelude::{interaction::Interaction, Activity, GuildId, Message, Ready},
        user::OnlineStatus,
    },
    prelude::{Context, EventHandler},
};
use sqlx::MySqlPool;

use crate::Database;

use self::{commands::handle_commands, reactions::react};

/// Retrieves the database from context
async fn get_database(ctx: &Context) -> Arc<MySqlPool> {
    let reader = ctx.data.read().await;
    reader.get::<Database>().unwrap().clone()
}

pub struct Events;

#[async_trait]
impl EventHandler for Events {
    async fn message(&self, ctx: Context, msg: Message) {
        react(&ctx, &msg).await;
    }
    async fn ready(&self, ctx: Context, _ready: Ready) {
        let activity = Activity::listening("YOU :3!");
        let status = OnlineStatus::Online;
        ctx.set_presence(Some(activity), status).await;

        create_commands(&ctx).await;
    }
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        // If the interaction is a command, handle it
        if let Interaction::ApplicationCommand(command) = interaction {
            handle_commands(ctx, command).await;
        }
    }
}

async fn create_commands(ctx: &Context) {
    let guild = GuildId(985827699853492274);
    guild
        .set_application_commands(&ctx.http, setup_commands)
        .await
        .map(|commands| {
            commands
                .into_iter()
                .map(|command| command.name)
                .for_each(|name| info!("Created command: {name}"));
        })
        .unwrap_or_else(|err| error!("Cannot create application commands: {err}"));
}

/// Registers all the commands used
fn setup_commands(commands: &mut CreateApplicationCommands) -> &mut CreateApplicationCommands {
    commands
        .create_application_command(commands::ignore::register)
        .create_application_command(commands::focus::register)
        .create_application_command(commands::cat_stuff::cuddle::register)
        .create_application_command(commands::cat_stuff::pet::register)
}
