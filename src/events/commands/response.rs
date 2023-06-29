use log::{error, info};
use serenity::{
    async_trait,
    model::prelude::interaction::{
        application_command::ApplicationCommandInteraction, InteractionResponseType,
    },
    prelude::Context,
    utils::Color,
};

use super::{
    focus::{FocusError, RemovedChannel},
    ignore::{AddedChannel, IgnoreError},
};

#[async_trait]
pub trait Response {
    async fn create_response(self, ctx: &Context, command: ApplicationCommandInteraction);
}

#[async_trait]
impl<T, E> Response for Result<T, E>
where
    T: Response + std::marker::Send + ToString,
    E: Response + std::marker::Send + ToString,
{
    async fn create_response(self, ctx: &Context, command: ApplicationCommandInteraction) {
        match self {
            Ok(res) => res.create_response(ctx, command).await,
            Err(err) => err.create_response(ctx, command).await,
        };
    }
}

#[async_trait]
impl Response for AddedChannel {
    async fn create_response(self, ctx: &Context, command: ApplicationCommandInteraction) {
        info!("Added channel - {}, to ignore list", self.to_string());
        command
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|content| {
                        content.embed(|embed| {
                            embed
                                .title("Added channel to ignore list!")
                                .description(format!("Added {} to ignore list", self.to_string()))
                                .colour(Color::DARK_GREEN)
                        })
                    })
            })
            .await
            .unwrap();
    }
}

#[async_trait]
impl Response for IgnoreError {
    async fn create_response(self, ctx: &Context, command: ApplicationCommandInteraction) {
        error!("Could not add channel to ignore list. {}", self.to_string());
        command
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|content| {
                        content.embed(|embed| {
                            embed
                                .title("ERROR!")
                                .description(format!(
                                    "Cannot add channel to ignore list - {}",
                                    self.to_string()
                                ))
                                .colour(Color::DARK_RED)
                        })
                    })
            })
            .await
            .unwrap();
    }
}

#[async_trait]
impl Response for RemovedChannel {
    async fn create_response(self, ctx: &Context, command: ApplicationCommandInteraction) {
        info!("Removed channel - {}, from ignore list", self.to_string());
        command
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|content| {
                        content.embed(|embed| {
                            embed
                                .title("Added channel to ignore list!")
                                .description(format!(
                                    "Removed {} from ignore list",
                                    self.to_string()
                                ))
                                .colour(Color::DARK_GREEN)
                        })
                    })
            })
            .await
            .unwrap();
    }
}

#[async_trait]
impl Response for FocusError {
    async fn create_response(self, ctx: &Context, command: ApplicationCommandInteraction) {
        error!(
            "Could not remove channel from ignore list. {}",
            self.to_string()
        );
        command
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|content| {
                        content.embed(|embed| {
                            embed
                                .title("ERROR!")
                                .description(format!(
                                    "Cannot unignore channel - {}",
                                    self.to_string()
                                ))
                                .colour(Color::DARK_RED)
                        })
                    })
            })
            .await
            .unwrap();
    }
}
