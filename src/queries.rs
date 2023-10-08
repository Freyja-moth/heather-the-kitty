pub const INSERT_CHANNEL: &str = "INSERT INTO ignored_channels(channel_id, guild_id) VALUES(?, ?)";
pub const REMOVE_CHANNEL: &str = "DELETE FROM ignored_channels WHERE channel_id = ?";
pub const SELECT_IGNORED: &str = "SELECT channel_id FROM ignored_channels WHERE guild_id = ?";
pub const IS_IGNORED: &str = "SELECT channel_id FROM ignored_channels WHERE channel_id = ?";
