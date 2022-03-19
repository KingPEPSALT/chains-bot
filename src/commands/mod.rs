use std::num::ParseIntError;

pub mod ping;
pub mod snapshot;
pub mod snapshot_channel;
pub mod mod_role;
pub mod disclaimer;
pub mod watch;

pub fn parse_channel(channel_mention: String) -> Result<i64, ParseIntError>{
    match channel_mention.parse::<i64>() {
        Ok(t) => Ok(t),
        Err(_) => channel_mention[2..channel_mention.len()-1].parse::<i64>()
    }
}