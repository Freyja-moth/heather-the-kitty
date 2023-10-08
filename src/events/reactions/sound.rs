use crate::prelude::*;

#[derive(Debug)]
pub enum Sound {
    Meow,
    Nyah,
}
impl Distribution<Sound> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Sound {
        match rng.gen_range(1..=2) {
            1 => Sound::Meow,
            _ => Sound::Nyah,
        }
    }
}
impl ToString for Sound {
    fn to_string(&self) -> String {
        format!("{self:?}")
    }
}
#[async_trait]
impl Respond for Sound {
    async fn respond(self, message: &Message, http: &Http) -> KittyResult {
        info!("I meowed at someone!");
        message
            .channel_id
            .send_message(http, |message| message.content(self))
            .await
            .map_err(ReactionError::CouldNotMakeASound)?;

        Ok(())
    }
}
