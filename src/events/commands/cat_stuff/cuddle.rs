use log::{error, info};
use rand::random;
use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::interaction::application_command::ApplicationCommandInteraction,
    prelude::Context,
};

use super::Reaction;

impl Reaction {
    fn into_cuddle_reaction(&self) -> String {
        match random() {
            Reaction::Anger => "Heather didn't want cuddles, so she tries to bite you!",
            Reaction::Ignore => "Heather didn't want cuddles, so she walks away",
            Reaction::Happy => "Heather loved the cuddle!",
        }
        .to_string()
    }
}

pub async fn run(command: ApplicationCommandInteraction, ctx: &Context) {
    if command
        .create_interaction_response(&ctx.http, |response| {
            response.interaction_response_data(|content| {
                content.content(random::<Reaction>().into_cuddle_reaction())
            })
        })
        .await
        .is_err()
    {
        error!("Sometime tried to cuddle me, but they missed!");
    } else {
        info!("Someone pet me!");
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("cuddle").description("Gives heather a cuddle")
}
