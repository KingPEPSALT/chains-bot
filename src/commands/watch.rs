
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    prelude::Context
};
use crate::{db::{add_watched_member, remove_watched_member}, WatchMemberHandler, WatchChannelHandler, commands::enforce_compliancy};
#[command] 
#[num_args(1)]
#[aliases(monitor)]
async fn watch(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult{
    let guild = *msg.guild_id.unwrap().as_u64();

    if !enforce_compliancy(ctx, msg, guild).await.0{
        return Ok(())
    }

    let user = match args.single::<u64>() {
        Ok(t) => t,
        Err(_) => {
            if msg.mentions.len() == 0{
                msg.reply(ctx, "You did not provide a sufficient ID or mention a user.").await?;
            }
            *msg.mentions[0].id.as_u64()
        }
    };
    let mut data = ctx.data.write().await;
    if *data.get::<WatchChannelHandler>().unwrap().get(&guild).unwrap() == 0{
        msg.reply(ctx, "There is no watch channel set for this server.").await?;
        return Ok(())
    }

    let cache = data.get_mut::<WatchMemberHandler>().unwrap();
    if let Some(users) = cache.get(&guild){
        if users.contains(&user){
            remove_watched_member(&guild, &user, cache)?;
        }else{
            add_watched_member(&guild, &user, cache)?;
        }
    };
    msg.delete(ctx).await?;
    Ok(())
}