#[derive(Debug)]
pub struct Guild{
    pub guild_id: u64,
    pub logging_channel_id: Option<u64>,
    pub moderation_role_id: Option<u64>,
}