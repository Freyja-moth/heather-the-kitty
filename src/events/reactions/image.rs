const HAPPY: &str = "./images/transcat-happy.png";
const SLEPY: &str = "./images/transcat-slepy.png";

use crate::prelude::*;

pub enum Image {
    Happy,
    Sleepy,
}
impl Distribution<Image> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Image {
        match rng.gen_range(1..=2) {
            1 => Image::Sleepy,
            _ => Image::Happy,
        }
    }
}
impl<'a> From<Image> for AttachmentType<'a> {
    fn from(value: Image) -> Self {
        Self::Path(Path::new(match value {
            Image::Happy => HAPPY,
            Image::Sleepy => SLEPY,
        }))
    }
}
#[async_trait]
impl Respond for Image {
    async fn respond(self, message: &Message, http: &Http) -> KittyResult {
        info!("Someone looked at me, I must be pretty!");
        message
            .channel_id
            .send_message(http, |message| message.add_file(AttachmentType::from(self)))
            .await
            .map_err(ReactionError::CouldNotMakeASound)?;

        Ok(())
    }
}
