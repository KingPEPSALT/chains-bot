#[derive(Debug)]
pub struct Guild{
    pub guild_id: u64,
    pub snapshot_channel: Option<u64>,
    pub mod_role: Option<u64>,
}