mod pet;

use log::{error, info};
use serenity::{
    async_trait,
    model::{
        application::interaction::Interaction,
        prelude::{interaction::InteractionResponseType, GuildId, Ready},
    },
    prelude::{Context, EventHandler},
};

pub struct Commands;

#[async_trait]
impl EventHandler for Commands {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        let Interaction::ApplicationCommand(command) = interaction else {
            return;
        };
        let context = match command.data.name.as_str() {
            "pet" => pet::run(),
            _ => "Error! Not a command".to_string(),
        };

        if let Err(why) = command
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| message.content(context))
            })
            .await
        {
            error!("Cannot respond to slash command: {why}");
        }
    }
    async fn ready(&self, ctx: Context, _: Ready) {
        let guild_id = GuildId(985827699853492274);

        let commands = guild_id
            .set_application_commands(&ctx.http, |commands| {
                commands.create_application_command(pet::register)
            })
            .await;
        info!("Created commands: {commands:#?}")
    }
}
