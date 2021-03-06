use crate::db::update_snapshot_channel;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    prelude::Context,
};
#[command]
#[num_args(1)]
#[aliases(set_snapshot_channel, snap_channel, set_snap_channel)]
async fn snapshot_channel(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult{

    if !msg.member(ctx).await.unwrap().permissions(ctx).await.unwrap().administrator(){
        msg.reply(ctx, "You need administrator for this command.").await?;
        return Ok(());
    }

    let channel_mention = args.single::<String>().unwrap();
    
    // grab the <#id> channel mention and parse the id part of it as a u64 integer
    let channel_id = match channel_mention[2..channel_mention.len()-1].parse::<u64>(){
        Ok(id) => id,
        Err(_) => {
            msg.reply(ctx, "That is not a valid channel.").await?;
            return Ok(());
        }
    };

    match update_snapshot_channel(&msg.guild_id.unwrap().as_u64(), &channel_id) {
        Ok(_) => msg.reply(ctx, format!("Successfully set channel to `{}`", channel_mention)).await?,
        Err(_) => msg.reply(ctx, "Could not set the snapshot channel, this is a fault with my code.").await?
    };

    Ok(())

}