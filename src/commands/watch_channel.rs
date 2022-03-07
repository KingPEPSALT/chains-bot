
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    prelude::Context
};
use crate::{db::add_watch_channel, WatchChannelHandler, commands::parse_channel};
#[command] 
#[min_args(1)]
#[max_args(2)]
#[aliases(monitor_channel)]
async fn watch_channel(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult{

    if !msg.member(ctx).await.unwrap().permissions(ctx).await.unwrap().administrator(){
        msg.reply(ctx, "You need administrator for this command.").await?;
        return Ok(());
    }

    let channel_id = match parse_channel(args.single::<String>().unwrap()){
        Ok(id) => id,
        Err(_) => {
            msg.reply(ctx, "That is not a valid channel ID or channel mention.").await?;
            return Ok(())
        }
    };
    
    let mut data = ctx.data.write().await;
    match add_watch_channel(&msg.guild_id.unwrap().as_u64(), &channel_id, data.get_mut::<WatchChannelHandler>().unwrap()){
        Ok(_) => msg.reply(ctx, format!("Successfully set channel to `<#{}>`", channel_id)).await?,
        Err(_) => msg.reply(ctx, "Could not set the snapshot channel, this is a fault with my code.").await?
    };
    Ok(())
}