use crate::prelude::*;

pub type KittyResult<T = ()> = Result<T, KittyError>;

#[derive(Error, Debug)]
pub enum KittyError {
    #[error(transparent)]
    Config(#[from] ConfigError),
    #[error(transparent)]
    Discord(#[from] DiscordError),
    #[error(transparent)]
    Database(#[from] DatabaseError),
    #[error(transparent)]
    Reaction(#[from] ReactionError),
    #[error(transparent)]
    Command(#[from] CommandError),
    #[error(transparent)]
    Affection(#[from] AffectionError),
}
impl KittyError {
    pub async fn send_error_response(
        &self,
        http: impl AsRef<Http>,
        interaction: &ApplicationCommandInteraction,
    ) -> KittyResult {
        interaction
            .create_interaction_response(http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| {
                        message.ephemeral(true).embed(|embed| {
                            embed
                                .title("Error")
                                .description(format!("{self}"))
                                .field("", "If the issue persists, please raise an issue here: https://github.com/Andrea-moth/heather-the-kitty/issues", true)
                                .color(Colour::RED)
                                .author(|author| {
                                    author.name("Freyja-moth").icon_url("https://avatars.githubusercontent.com/u/103472619?v=4")
                                })
                        })
                    })
            })
            .await
            .map_err(DiscordError::CannotSendResponse)?;

        error!("{self}");

        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum DiscordError {
    #[error("Cannot build bot, why: {0}")]
    CannotBuildBot(SerenityError),
    #[error("Cannot start bot, why: {0}")]
    CannotStartBot(SerenityError),
    #[error("Cannot send response: {0}")]
    CannotSendResponse(SerenityError),
}

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Cannot setup logging: {0}")]
    CannotSetupLogging(log::SetLoggerError),
    #[error("Cannot load enviroment: {0}")]
    CannotLoadEnviroment(dotenv::Error),
    #[error("Cannot find config file: {0}")]
    CannotLoadConfig(io::Error),
    #[error("Cannot parse config file: {0}")]
    CannotParseConfig(toml::de::Error),
    #[error("Cannot save config: {0}")]
    CannotSaveConfig(toml::ser::Error),
    #[error("Cannot parse commands: {0}")]
    CannotParseCommands(clap::Error),
}

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Cannot connect to database: {0}")]
    CannotConnectToDatabase(sqlx::Error),
    #[error("Cannot retrieve database")]
    CannotRetrieveDatabase,
    #[error("Unable to insert channel to table: {0}")]
    UnableToInsertChannelToTable(sqlx::Error),
    #[error("Unable to remove channel from table: {0}")]
    UnableToRemoveChannelFromTable(sqlx::Error),
    #[error("Unable to check if channel is ignored: {0}")]
    UnableToCheckIfChannelIsIgnored(sqlx::Error),
    #[error("Unable to find ignored channels: {0}")]
    UnableToFindIgnoredChannels(sqlx::Error),
}

#[derive(Error, Debug)]
pub enum ReactionError {
    #[error("Could not make a sound")]
    CouldNotMakeASound(SerenityError),
    #[error("Could not send image")]
    CouldNotSendImage(SerenityError),
}

#[derive(Error, Debug)]
pub enum CommandError {
    #[error("Cannot create command: {0}")]
    CannotCreateCommand(SerenityError),
    #[error("Cannot find command option")]
    CannotFindCommandOption,
    #[error("Cannot retrieve option value")]
    CannotRetrieveOptionValue,
    #[error("Option value is of wrong type")]
    OptionValueIsOfWrongType,
    #[error("Command ran outside of guild")]
    CommandRanOutsideOfGuild,
    #[error("Tried to run non registered command")]
    CannotFindCommand,
    #[error("Channel is already ignored")]
    ChannelIsAlreadyIgnored,
    #[error("Channel is already focused")]
    ChannelIsAlreadyFocused,
    #[error("No channels found")]
    NoChannelsFound,
}

#[derive(Error, Debug)]
pub enum AffectionError {
    #[error("Channel is been ignored")]
    ChannelIsIgnored,
    #[error("Could not generate hug response")]
    CouldNotGenerateHugResponse,
    #[error("Could not respond to hug")]
    CouldNotRespondToHug(SerenityError),
    #[error("Could not generate pet response")]
    CouldNotGeneratePetResponse,
    #[error("Could not respond to pats")]
    CouldNotRespondToPat(SerenityError),
    #[error("Could not generate scratch response")]
    CouldNotGenerateScratchResponse,
    #[error("Could not respond to scratch")]
    CouldNotRespondToScratch(SerenityError),
}
