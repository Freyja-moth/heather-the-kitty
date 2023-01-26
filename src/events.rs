use log::{error, info};
use rand::{distributions::Standard, prelude::Distribution, Rng};
use serenity::{
    async_trait,
    model::prelude::{Message, Ready},
    prelude::{Context, EventHandler},
};

use crate::chance;

enum ReplyTypes {
    Sound(Sounds),
    Image(Image),
}
impl Distribution<ReplyTypes> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ReplyTypes {
        match rng.gen_range(0..=3) {
            0 | 1 | 2 => ReplyTypes::Sound(rand::random::<Sounds>()),
            3 => ReplyTypes::Image(rand::random::<Image>()),
            _ => panic!("This should never happen and if it does you have severly fucked yourself, search for [impl Distribution<ReplyTypes> for Standard] to find where it failed")
        }
    }
}

enum Sounds {
    Meow,
    Nyaa,
    Mrurp,
}
impl Distribution<Sounds> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Sounds {
        match rng.gen_range(0..=2) {
            0 => Sounds::Meow,
            1 => Sounds::Nyaa,
            2 => Sounds::Mrurp,
            //2 => ReplyTypes::Image,
            _ => panic!("This should never happen and if it does you have severly fucked yourself, search for [impl Distribution<Sounds> for Standard] to find where it failed")
        }
    }
}
impl Sounds {
    fn to_string(&self) -> String {
        match self {
            Self::Meow => "Meow".to_string(),
            Self::Nyaa => "Nyaa".to_string(),
            Self::Mrurp => "Mrurp".to_string(),
        }
    }
}

enum Image {
    Happy,
    Sleep,
}
impl Distribution<Image> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Image {
        match rng.gen_range(0..=1) {
            0 => Image::Happy,
            1 => Image::Sleep,
            _ => panic!("This should never happen and if it does you have severly fucked yourself, search for [impl Distribution<Image> for Standard] to find where it failed")
        }
    }
}
impl Image {
    fn to_path<'a>(&self) -> &'a str {
        match self {
            Image::Happy => "./images/transcat-happy.png",
            Image::Sleep => "./images/transcat-slepy.png",
        }
    }
}

const EVENT_CHANCE: f64 = 0.5;
const SERIOUS_CHANNELS: &[u64] = &[];

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        } else if msg.content.starts_with("~") {
            return;
        } else if SERIOUS_CHANNELS.contains(msg.channel_id.as_u64()) {
            return;
        } else if chance() > EVENT_CHANCE {
            info!("Someone talked to me, but I didn't feel like doing anything");
            return;
        }

        let mut text: Option<String> = None;
        let mut file: Option<&str> = None;

        match rand::random::<ReplyTypes>() {
            ReplyTypes::Sound(sound) => {
                info!("Meowed at someone!");
                text = Some(sound.to_string());
            }
            ReplyTypes::Image(image) => {
                info!("Someone looked at me, I must be pretty!");
                file = Some(image.to_path());
            }
        };
        if let Err(_) = msg
            .channel_id
            .send_message(&ctx.http, |m| {
                m.content(if let Some(sound) = text {
                    sound
                } else {
                    "".to_string()
                });
                if let Some(file) = file {
                    m.add_file(file)
                } else {
                    m
                }
            })
            .await
        {
            error!("Someone talked to me, but I didn't feel like doing anything");
        }
    }

    async fn ready(&self, _ctx: Context, _ready: Ready) {
        info!("Thoughts of cat")
    }
}
