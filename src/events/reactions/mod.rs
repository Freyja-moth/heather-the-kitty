mod image;
mod sound;

use log::{error, info};
use rand::{distributions::Standard, prelude::Distribution, random};
use serenity::{model::prelude::Message, prelude::Context};

use self::{
    image::{image, Image},
    sound::{sound, Sound},
};

use super::get_database;

pub async fn react(ctx: &Context, msg: &Message) {
    let pool = get_database(ctx).await;
    let channel_id = msg.channel_id.to_string();

    let is_ignored_channel = sqlx::query!(
        "SELECT channel_id FROM ignore_channels WHERE channel_id = ?",
        channel_id
    )
    .fetch_one(&*pool)
    .await
    .is_ok();
    let is_bot = msg.author.bot;

    if is_ignored_channel || is_bot {
        info!("I was going to say something, but they seemed busy");
        return;
    }

    // Gets a random reaction and; if it's not none, passes it into the function
    if let Err(why) = match random() {
        Reaction::Sound(sound_made) => sound(ctx, msg, sound_made).await,
        Reaction::Image(image_sent) => image(ctx, msg, image_sent).await,
        Reaction::None => Ok(()),
    } {
        error!("I tried to do something, but it didn't work. {why}");
    }
}

enum Reaction {
    Sound(Sound),
    Image(Image),
    None,
}
impl Distribution<Reaction> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Reaction {
        match rng.gen_range(0..=128) {
            1..=32 => Reaction::Sound(random::<Sound>()),
            33..=64 => Reaction::Image(random::<Image>()),
            _ => Reaction::None,
        }
    }
}
