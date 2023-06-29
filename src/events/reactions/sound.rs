use log::info;
use rand::{distributions::Standard, prelude::Distribution};
use serenity::{model::prelude::Message, prelude::Context};

use crate::error::CatResult;

pub async fn sound(ctx: &Context, msg: &Message, sound: Sound) -> CatResult<()> {
    msg.channel_id
        .send_message(&ctx.http, |message| message.content(sound))
        .await?;
    info!("I meowed at someone!");
    Ok(())
}

/// Random sound to be made
pub enum Sound {
    /// Heather meows
    Meoww,
    /// Heather mrupps
    Mrupp,
    /// Heather nyahhs
    Nyahhh,
}
impl Distribution<Sound> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Sound {
        match rng.gen_range(0..=2) {
            0 => Sound::Meoww,
            1 => Sound::Mrupp,
            _ => Sound::Nyahhh,
        }
    }
}
impl ToString for Sound {
    fn to_string(&self) -> String {
        match *self {
            Sound::Meoww => "Meoww!",
            Sound::Mrupp => "Mrurp!",
            Sound::Nyahhh => "Nyahh!",
        }
        .to_string()
    }
}
