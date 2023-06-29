use log::{error, info};
use rand::{distributions::Standard, prelude::Distribution, random};
use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::interaction::application_command::ApplicationCommandInteraction,
    prelude::Context,
};

enum Reaction {
    Happy,
    Ignore,
    Anger,
}
impl Distribution<Reaction> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Reaction {
        match rng.gen_range(0..=64) {
            0..=3 => Reaction::Anger,
            4..=6 => Reaction::Ignore,
            _ => Reaction::Happy,
        }
    }
}
impl ToString for Reaction {
    fn to_string(&self) -> String {
        match random() {
            Reaction::Anger => "Heather didn't want pets, so she tries to bite you!",
            Reaction::Ignore => "Heather didn't want pets, so she walks away",
            Reaction::Happy => "Heather loved the pets!",
        }
        .to_string()
    }
}

pub async fn run(command: ApplicationCommandInteraction, ctx: &Context) {
    if command
        .create_interaction_response(&ctx.http, |response| {
            response.interaction_response_data(|content| content.content(random::<Reaction>()))
        })
        .await
        .is_err()
    {
        error!("Someone tried to pet me, but they missed!");
    } else {
        info!("Someone tried to pet me!");
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("pet").description("Gives heather a pat")
}
