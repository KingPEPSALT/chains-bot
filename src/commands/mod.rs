use std::num::ParseIntError;

use serenity::{model::channel::Message, client::Context};
use crate::db::{get_guild, model::Guild};

pub mod ping;
pub mod snapshot;
pub mod snapshot_channel;
pub mod mod_role;
pub mod disclaimer;
pub mod watch_channel;
pub mod watch;

pub async fn enforce_compliancy(ctx: &Context, msg: &Message, guild: u64) -> (bool, Option<Guild>){
    let request = match get_guild(&guild) 
    {
        Ok(guild_model) => guild_model,
        Err(e) => {
            msg.reply(ctx, format!("Could not get guild from database | {} | This is an error within the code.", e.to_string())).await.expect("Could not enforce disclaimer.");
            return (false, None);
        },
    };
    
    if ! request.disclaimer_compliant {

        msg.reply(ctx, format!("A server admin must accept the `{}disclaimer`", dotenv::var("DISCORD_PREFIX").unwrap())).await.expect("Could not enforce disclaimer.");
        return (false, Some(request));
    }
    (true, Some(request))
}

pub fn parse_channel(channel_mention: String) -> Result<u64, ParseIntError>{
    
    match channel_mention[2..channel_mention.len()-1].parse::<u64>(){
        Ok(id) => Ok(id),
        Err(_) => {
            channel_mention.parse::<u64>()
        }
    }
}