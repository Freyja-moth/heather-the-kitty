use crate::prelude::*;

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("hug").description("Gives heather a hug.")
}

const HUG_RESPONSES: [&str; 2] = [
    "Heather absolutely loved the hug",
    "You gave heather the tightest hug",
];

pub async fn run(
    interaction: &ApplicationCommandInteraction,
    http: impl AsRef<Http>,
) -> KittyResult {
    let hug_response = HUG_RESPONSES
        .choose(&mut rand::thread_rng())
        .ok_or(AffectionError::CouldNotGenerateHugResponse)?;

    interaction
        .create_interaction_response(http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| message.content(hug_response))
        })
        .await
        .map_err(AffectionError::CouldNotRespondToHug)?;

    Ok(())
}
