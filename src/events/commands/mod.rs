pub mod cat_stuff;
pub mod focus;
pub mod ignore;

mod response;

use serenity::{
    model::application::interaction::application_command::ApplicationCommandInteraction,
    prelude::Context,
};

pub async fn handle_commands(ctx: Context, command: ApplicationCommandInteraction) {
    match command.data.name.as_str() {
        "ignore" => ignore::run(command, &ctx).await,
        "focus" => focus::run(command, &ctx).await,
        "cuddle" => cat_stuff::cuddle::run(command, &ctx).await,
        "pet" => cat_stuff::pet::run(command, &ctx).await,
        _ => {}
    }
}
