
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    prelude::Context
};
use crate::{db::add_watch_channel, WatchChannelHandler};
#[command] 
#[min_args(1)]
#[max_args(2)]
#[aliases(monitor_channel)]
async fn watch_channel(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult{
    let chan = args.single::<u64>().unwrap();
    let mut data = ctx.data.write().await;
    match add_watch_channel(&msg.guild_id.unwrap().as_u64(), &chan, data.get_mut::<WatchChannelHandler>().unwrap()){
        Err(_) => {
            msg.reply(&ctx.http, "Could not set the channel.").await?;
        },
        Ok(_) => {
            msg.delete(&ctx.http).await?;
        }
    }
    Ok(())
}