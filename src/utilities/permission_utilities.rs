use db::sea_orm::EntityTrait;
use serenity::{model::channel::Message, client::Context};

use crate::Connection;

// this effectively consumes errors and will always return true or false
pub async fn is_message_author_admin(ctx: &Context, msg: &Message) -> bool {

    let member =  msg.member(ctx).await.unwrap();
    
    let role_id = match db::guild::Entity::find_by_id(*msg.guild_id.unwrap().as_u64() as i64)
        .one(ctx.data.read().await.get::<Connection>()
        .expect("Connection to database does not exist.")).await {
            Ok(Some(x)) => x.moderation_role_id,
            _ => return false
        };

    if member.permissions(ctx).await.unwrap().administrator(){
        return true;
    }
    if role_id.is_none(){
        return false
    }
        
    member.roles(ctx).await
        .expect("Expected role list").iter().find(|&r| *r.id.as_u64() as i64 == role_id.unwrap()).is_some()
}

pub async fn enforce_request_compliancy(ctx: &Context, msg: &Message) -> Option<db::guild::Model>{
    let request = match db::guild::Entity::find_by_id(*msg.guild_id.unwrap().as_u64() as i64)
        .one(ctx.data.read().await.get::<Connection>()
        .expect("Connection to database does not exist.")).await
    {
        Ok(Some(guild_model)) => guild_model,
        _ => {
            msg.reply(ctx, "Could not get guild from database | This is an error within the code.").await.expect("Could not enforce disclaimer.");
            return None;
        }
    };
    
    if ! request.is_compliant {
        msg.reply(ctx, format!("A server admin must accept the `{}disclaimer`", dotenv::var("DISCORD_PREFIX").unwrap())).await.expect("Could not enforce disclaimer.");
        return Some(request);
    }
    Some(request)
}
