use std::num::ParseIntError;

use db::sea_orm::EntityTrait;
use serenity::{model::channel::Message, client::Context};

use crate::Connection;

pub mod ping;
pub mod snapshot;
pub mod snapshot_channel;
pub mod mod_role;
pub mod disclaimer;
pub mod watch;

pub async fn enforce_compliancy(ctx: &Context, msg: &Message) -> (bool, Option<db::guild::Model>){
    let request = match db::guild::Entity::find_by_id(*msg.guild_id.unwrap().as_u64() as i64)
        .one(ctx.data.read().await.get::<Connection>()
        .expect("Connection to database does not exist.")).await
    {
        Ok(Some(guild_model)) => guild_model,
        Ok(None) => {
            msg.reply(ctx, "Could not get guild from database | This is an error within the code.").await.expect("Could not enforce disclaimer.");
            return (false, None);
        }
        Err(e) => {
            msg.reply(ctx, format!("Could not get guild from database | {} | This is an error within the code.", e.to_string())).await.expect("Could not enforce disclaimer.");
            return (false, None);
        },
    };
    
    if ! request.is_compliant {

        msg.reply(ctx, format!("A server admin must accept the `{}disclaimer`", dotenv::var("DISCORD_PREFIX").unwrap())).await.expect("Could not enforce disclaimer.");
        return (false, Some(request));
    }
    (true, Some(request))
}

pub async fn is_moderator(ctx: &Context, msg: &Message) -> bool{

    let member =  msg.member(ctx).await.unwrap();
    
    let role_id = db::guild::Entity::find_by_id(*msg.guild_id.unwrap().as_u64() as i64)
        .one(ctx.data.read().await.get::<Connection>()
        .expect("Connection to database does not exist.")).await.unwrap().unwrap().moderation_role_id;

    if member.permissions(ctx).await.unwrap().administrator(){
        return true;
    }
    if role_id.is_none(){
        return false
    }
        
    member.roles(ctx).await
        .expect("Expected role list").iter().find(|&r| *r.id.as_u64() as i64 == role_id.unwrap()).is_some()
}

pub fn parse_channel(channel_mention: String) -> Result<i64, ParseIntError>{
    match channel_mention.parse::<i64>() {
        Ok(t) => Ok(t),
        Err(_) => channel_mention[2..channel_mention.len()-1].parse::<i64>()
    }
}