use std::path::Path;

use log::info;
use rand::{distributions::Standard, prelude::Distribution};
use serenity::{
    model::prelude::{AttachmentType, Message},
    prelude::Context,
};

use crate::error::CatResult;

pub async fn image(ctx: &Context, msg: &Message, image: Image) -> CatResult<()> {
    msg.channel_id
        .send_message(&ctx.http, |message| message.add_file(image))
        .await?;
    info!("Someone looked at me! I must be pretty");
    Ok(())
}

const HAPPY_CAT: &str = "./images/transcat-happy.png";
const SLEEPY_CAT: &str = "./images/transcat-slepy.png";

/// Random image to be sent
pub enum Image {
    /// Heather is happy
    Happy,
    /// Heather is eepy
    Sleepy,
}
impl Distribution<Image> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Image {
        match rng.gen_range(0..=1) {
            0 => Image::Happy,
            _ => Image::Sleepy,
        }
    }
}
impl<'a> From<Image> for AttachmentType<'a> {
    fn from(val: Image) -> Self {
        AttachmentType::Path(Path::new(match val {
            Image::Happy => HAPPY_CAT,
            Image::Sleepy => SLEEPY_CAT,
        }))
    }
}
