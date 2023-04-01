use log::{error, info};
use std::path::Path;

use rand::{distributions::Standard, prelude::Distribution, random};
use serenity::{
    async_trait,
    http::AttachmentType,
    model::prelude::{Message, Ready},
    prelude::{Context, EventHandler},
};

use crate::error::CatResult;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected", ready.user.name);
    }
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }
        if let Err(why) = match random() {
            ResponseType::Nothing => self.nothing().await,
            ResponseType::Sound => self.sound(ctx, msg).await,
            ResponseType::Image => self.image(ctx, msg).await,
        } {
            error!("Couldn't respond {why}");
        }
    }
}

impl Handler {
    async fn sound(&self, ctx: Context, msg: Message) -> CatResult<()> {
        info!("Meowed at someone!");
        let sound: Sound = random();
        msg.channel_id
            .send_message(&ctx.http, |message| message.content(sound))
            .await?;
        Ok(())
    }
    async fn image(&self, ctx: Context, msg: Message) -> CatResult<()> {
        info!("Sent someone an image");
        let image: Image = random();
        msg.channel_id
            .send_message(&ctx.http, |message| message.add_file(image))
            .await?;
        Ok(())
    }
    async fn nothing(&self) -> CatResult<()> {
        info!("Someone tried to talk to me... but I ignored them");
        Ok(())
    }
}

enum ResponseType {
    Nothing,
    Sound,
    Image,
}
impl Distribution<ResponseType> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> ResponseType {
        match rng.gen_range(0..=64) {
            0..=30 => ResponseType::Image,
            31..=60 => ResponseType::Sound,
            _ => ResponseType::Nothing,
        }
    }
}

enum Sound {
    Meow,
    Nya,
    Mrurp,
}
impl Distribution<Sound> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Sound {
        match rng.gen_range(0..=2) {
            0 => Sound::Meow,
            1 => Sound::Nya,
            _ => Sound::Mrurp,
        }
    }
}
impl ToString for Sound {
    fn to_string(&self) -> String {
        match self {
            Self::Meow => "Meow!",
            Self::Nya => "Nya!",
            Self::Mrurp => "Mrurp!",
        }
        .to_string()
    }
}

const HAPPY_CAT: &str = "images/transcat-happy.png";
const SLEEPY_CAT: &str = "image/transcat-sleepy.png";

enum Image {
    Happy,
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
impl<'a> Into<AttachmentType<'a>> for Image {
    fn into(self) -> AttachmentType<'a> {
        match self {
            Image::Happy => AttachmentType::Path(Path::new(HAPPY_CAT)),
            Image::Sleepy => AttachmentType::Path(Path::new(SLEEPY_CAT)),
        }
    }
}
