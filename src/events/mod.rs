pub mod commands;
pub mod reactions;

use crate::prelude::*;

pub struct Events;

#[async_trait]
impl EventHandler for Events {
    async fn ready(&self, ctx: Context, _ready: Ready) {
        info!("Heather is now online");

        if let Err(err) = try_ready(&ctx).await {
            error!("{err}");
        }
    }
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Err(err) = try_interaction(&ctx, &interaction).await {
            error!("{err}");
        }
    }

    async fn message(&self, ctx: Context, message: Message) {
        if let Err(err) = try_message(&ctx, &message).await {
            error!("{err}");
        }
    }
}

async fn try_ready(ctx: &Context) -> KittyResult {
    commands::create_commands(&ctx.http).await?;
    Ok(())
}

async fn try_interaction(ctx: &Context, interaction: &Interaction) -> KittyResult {
    commands::run_commands(ctx, interaction).await?;

    Ok(())
}

async fn try_message(ctx: &Context, message: &Message) -> KittyResult {
    if !message.author.bot {
        reactions::generate_response(ctx, message).await?;
    }
    Ok(())
}
