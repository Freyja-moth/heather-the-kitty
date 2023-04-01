use rand::{distributions::Standard, prelude::Distribution, random};
use serenity::{
    async_trait,
    model::prelude::{Message, Ready},
    prelude::{Context, EventHandler},
};

use crate::error::CatResult;

enum ResponseType {
    Nothing,
    Sound,
    Image,
}
impl Distribution<ResponseType> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> ResponseType {
        match rng.gen_range(0..=64) {
            //0..=30 => ResponseType::Image,
            0..=60 => ResponseType::Sound,
            _ => ResponseType::Nothing,
        }
    }
}

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected", ready.user.name);
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
            println!("Couldn't respond {why}");
        }
    }
}

impl Handler {
    async fn sound(&self, ctx: Context, msg: Message) -> CatResult<()> {
        println!("Meowed at someone!");
        let sound: Sound = random();
        msg.channel_id
            .send_message(&ctx.http, |message| message.content(sound))
            .await?;
        Ok(())
    }
    async fn image(&self, _ctx: Context, _msg: Message) -> CatResult<()> {
        todo!("I'm too lazy to work with images yet");
        //msg.channel_id
        //    .send_message(&ctx.http, |message| message.content("Image"))
        //    .await?;
        //Ok(())
    }
    async fn nothing(&self) -> CatResult<()> {
        println!("Someone tried to talk to me... but I ignored them");
        Ok(())
    }
}

enum Sound {
    Meow,
    Nya,
    Mrurp,
}
impl Distribution<Sound> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Sound {
        match rng.gen_range(0..=1) {
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
