use crate::chance;
use log::info;
use rand::seq::SliceRandom;
use serenity::{
    framework::standard::{
        macros::{command, group},
        CommandResult,
    },
    model::prelude::Message,
    prelude::Context,
};

const IGNORE_CHANCE: f64 = 0.04;
const IGNORE_FLAVOR_TEXT: &'static [&str] = &[
    "Heather doesn't seem all that interested in your shenanigans!",
    "Heather gives you a indignant look and flops over onto her side!",
];
fn rand_ignore_flavor_text() -> String {
    IGNORE_FLAVOR_TEXT
        .choose(&mut rand::thread_rng())
        .unwrap()
        .to_string()
}

const BITE_CHANCE: f64 = 0.03;
const BITE_FLAVOR_TEXT: &'static [&str] = &[
    "You haven't fed her in a while, so Heather decides to bite you",
    "Heather bites you, for no reason other than she's an asshole. But you love her anyways!",
];
fn rand_bite_flavor_text() -> String {
    BITE_FLAVOR_TEXT
        .choose(&mut rand::thread_rng())
        .unwrap()
        .to_string()
}

const BREAK_CHANCE: f64 = 0.01;
const BREAK_FLAVOR_TEXT: &'static [&str] = &[
    "You can't find heather anywhere, not even in her favourite hiding space! That is, until you hear a large crash in the other room... She's knocked over a vase!",
    "Heather ignores what ever it was you were trying to do and starts attacking the couch instead!",
    "Heather give you a disinterested look, then spits up a hairball!"
];
fn rand_break_flavor_text() -> String {
    BREAK_FLAVOR_TEXT
        .choose(&mut rand::thread_rng())
        .unwrap()
        .to_string()
}

const CUDDLE_FLAVOR_TEXT: &'static [&str] = &[
    "Heather snuggles up against you contendedly!",
    "Slinking over to you Heather *floops* onto your stomach!",
];
const PET_FLAVOR_TEXT: &'static [&str] = &[
    "Heather starts purring as you stroke her head!",
    "You can hear the rumbling of Heathers purrs as you scratch her chin!",
    "Heather *boops* herself into your outstretched hand!",
];
const YARN_FLAVOR_TEXT: &'static [&str] = &[
    "Scampering over to the recently tossed yarn Heather trips over her own paws and ends up tangled in it!",
    "Heather makes quick work of the yarn, tearing it into tiny shreads before walking away proudly"
];

#[group]
#[commands(cuddle, pet, yarn_ball)]
pub struct General;

#[command]
async fn cuddle(ctx: &Context, msg: &Message) -> CommandResult {
    basic_command(ctx, msg, CUDDLE_FLAVOR_TEXT).await?;
    info!("SOMEONE CUDDLED ME!");
    Ok(())
}

#[command]
async fn pet(ctx: &Context, msg: &Message) -> CommandResult {
    basic_command(ctx, msg, PET_FLAVOR_TEXT).await?;
    info!("SCRITCHES!!");
    Ok(())
}

#[command]
async fn yarn_ball(ctx: &Context, msg: &Message) -> CommandResult {
    basic_command(ctx, msg, YARN_FLAVOR_TEXT).await?;
    info!("Ohhh, yarn!");
    Ok(())
}

async fn basic_command(ctx: &Context, msg: &Message, flavor_text: &[&str]) -> CommandResult {
    if chance() < IGNORE_CHANCE {
        msg.reply(ctx, rand_ignore_flavor_text()).await?;
        info!("Someone did some stuff, but I didn't feel like listening!");
        return Ok(());
    } else if chance() < BITE_CHANCE {
        msg.reply(ctx, rand_bite_flavor_text()).await?;
        info!("Someone annoyed me, so I bit them!");
        return Ok(());
    } else if chance() < BREAK_CHANCE {
        info!("Ooooh, loud noise");
        msg.reply(ctx, rand_break_flavor_text()).await?;
        return Ok(());
    }

    let flavor = flavor_text
        .choose(&mut rand::thread_rng())
        .expect("You didn't provide flavor text")
        .to_string();

    msg.channel_id
        .send_message(&ctx.http, |m| m.content(flavor).reference_message(msg))
        .await?;
    Ok(())
}
