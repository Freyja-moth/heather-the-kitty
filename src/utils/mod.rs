use crate::prelude::*;

pub async fn succeded_response(
    interaction: &ApplicationCommandInteraction,
    http: impl AsRef<Http>,
    message_response: String,
) -> KittyResult {
    interaction
        .create_interaction_response(http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| {
                    message.ephemeral(true).embed(|embed| {
                        embed
                            .title("Success!")
                            .description(message_response)
                            .colour(Colour::DARK_GREEN)
                    })
                })
        })
        .await
        .map_err(DiscordError::CannotSendResponse)?;
    Ok(())
}
