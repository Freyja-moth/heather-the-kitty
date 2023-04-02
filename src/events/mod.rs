mod commands;

use log::{error, info};
use std::path::Path;

use rand::{distributions::Standard, prelude::Distribution, random};
use serenity::{
    async_trait,
    model::{
        channel::AttachmentType,
        prelude::{command::Command, interaction::Interaction, GuildId, Message, Ready},
    },
    prelude::{Context, EventHandler},
};

use crate::error::CatResult;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected", ready.user.name);
        match register_commands(ctx).await {
            Err(why) => error!("{why:?}"),
            Ok(commands) => commands
                .iter()
                .map(|command| command.name.as_str())
                .for_each(|command| info!("Created command: {command}")),
        }
    }
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }
        if let Err(why) = match random() {
            ResponseType::Nothing => nothing(),
            ResponseType::Sound => sound(ctx, msg).await,
            ResponseType::Image => image(ctx, msg).await,
        } {
            error!("Couldn't respond {why}");
        }
    }
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            if let Err(why) = match command.data.name.as_str() {
                "pet" => commands::pet::run(ctx, command).await,
                "cuddle" => commands::cuddle::run(ctx, command).await,
                _ => commands::error::run(ctx, command).await,
            } {
                error!("Cannot create response: {why:?}");
            }
        }
    }
}

async fn sound(ctx: Context, msg: Message) -> CatResult<()> {
    info!("Meowed at someone!");
    let sound: Sound = random();
    msg.channel_id
        .send_message(&ctx.http, |message| message.content(sound))
        .await?;
    Ok(())
}
async fn image(ctx: Context, msg: Message) -> CatResult<()> {
    info!("Sent someone an image");
    let image: Image = random();
    msg.channel_id
        .send_message(&ctx.http, |message| message.add_file(image))
        .await?;
    Ok(())
}
fn nothing() -> CatResult<()> {
    info!("Someone tried to talk to me... but I ignored them");
    Ok(())
}
async fn register_commands(ctx: Context) -> CatResult<Vec<Command>> {
    let guild_id = GuildId(985827699853492274);

    Ok(guild_id
        .set_application_commands(&ctx.http, |commands| {
            commands
                .create_application_command(commands::pet::register)
                .create_application_command(commands::cuddle::register)
        })
        .await?)
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
