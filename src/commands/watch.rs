
use db::sea_orm::{EntityTrait, ModelTrait, ActiveValue::Set, ActiveModelTrait};
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    prelude::Context
};
use super::enforce_compliancy;
use crate::{MemberCache, Connection, commands::{parse_channel, is_moderator}};

#[command] 
#[min_args(1)]
#[max_args(2)]
#[aliases(monitor)]
async fn watch(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult{
    if !is_moderator(ctx, msg).await{
        msg.reply(ctx, "You must be a moderator to run this command").await?;
        return Ok(())
    }


    let guild_id = *msg.guild_id.unwrap().as_u64() as i64;

    if !enforce_compliancy(ctx, msg).await.unwrap().is_compliant{
        return Ok(())
    }

    let user_id = match args.single::<u64>() {
        Ok(t) => t,
        Err(_) => {
            if msg.mentions.len() == 0{
                msg.reply(ctx, "You did not provide a sufficient ID or mention a user.").await?;
                return Ok(());
            }
            *msg.mentions[0].id.as_u64()
        }
    } as i64;

    // get the database connection
    let mut data = ctx.data.write().await;
    let con = data.get::<Connection>().unwrap();

    if let Some(Some(_)) = data.get::<MemberCache>().unwrap().get(&(guild_id, user_id)){
        // remove the watched member from the database and from the hashmap
        db::member::Entity::find_by_id((guild_id, user_id))
            .one(con).await.expect(&format!("Could not find member ({},{}) in members.", guild_id, user_id)).unwrap()
            .delete(con).await.expect(&format!("Could not delete member ({}, {}) from members.", guild_id, user_id));
        data.get_mut::<MemberCache>().unwrap().remove(&(guild_id, user_id));

    }else{
        // was a channel provided
        match parse_channel(args.advance().single::<String>().unwrap()) {
            Ok(ch) => {
                // add a channel to the database using sea_orm active models
                db::member::ActiveModel{
                    guild_id: Set(guild_id),
                    user_id: Set(user_id),
                    watch_channel_id: Set(Some(ch))
                }.insert(con).await
                .expect(&format!("Could not insert member ({}, {}) into members.", guild_id, user_id));
                data.get_mut::<MemberCache>().unwrap().insert((guild_id, user_id), Some(ch));
            },
            Err(_) => {
                msg.reply(ctx, "Please supply a channel to watch the user in.").await?;
                return Ok(());  
            }
        }
    }
    msg.delete(ctx).await?;
    Ok(())
}