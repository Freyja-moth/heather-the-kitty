use crate::prelude::*;

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("pat").description("Gives heather a pat.")
}

const PAT_RESPONSES: [&str; 2] = [
    "Heather absolutely loved the pat",
    "You gave heather the fluffiest pat",
];

pub async fn run(
    interaction: &ApplicationCommandInteraction,
    http: impl AsRef<Http>,
) -> KittyResult {
    let pat_response = PAT_RESPONSES
        .choose(&mut rand::thread_rng())
        .ok_or(AffectionError::CouldNotGeneratePetResponse)?;

    interaction
        .create_interaction_response(http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| message.content(pat_response))
        })
        .await
        .map_err(AffectionError::CouldNotRespondToPat)?;

    Ok(())
}
