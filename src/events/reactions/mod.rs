pub mod image;
pub mod sound;

use crate::prelude::*;

#[async_trait]
pub trait Respond {
    async fn respond(self, message: &Message, http: &Http) -> KittyResult;
}

pub enum ReactionType {
    Sound(Sound),
    Image(Image),
    Nothing,
}
impl Distribution<ReactionType> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> ReactionType {
        match rng.gen_range(1..=255) {
            1..=25 => ReactionType::Image(random()),
            26..=50 => ReactionType::Sound(random()),
            _ => ReactionType::Nothing,
        }
    }
}

pub async fn generate_response(ctx: &Context, message: &Message) -> KittyResult {
    match random() {
        ReactionType::Nothing => Ok(()),
        ReactionType::Sound(sound) => sound.respond(message, &ctx.http).await,
        ReactionType::Image(image) => image.respond(message, &ctx.http).await,
    }
}
