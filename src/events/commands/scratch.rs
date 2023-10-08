use crate::prelude::*;

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("scritches")
        .description("Gives heather scritches.")
        .default_member_permissions(Permissions::MANAGE_CHANNELS)
}

const SCRATCH_RESPONSES: [&str; 2] = [
    "Heather absolutely loved the scritches",
    "You gave heather the most satisfying scritches",
];

pub async fn run(
    interaction: &ApplicationCommandInteraction,
    http: impl AsRef<Http>,
) -> KittyResult {
    let scratch_response = SCRATCH_RESPONSES
        .choose(&mut rand::thread_rng())
        .ok_or(AffectionError::CouldNotGenerateHugResponse)?;

    interaction
        .create_interaction_response(http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| message.content(scratch_response))
        })
        .await
        .map_err(AffectionError::CouldNotRespondToHug)?;

    Ok(())
}
