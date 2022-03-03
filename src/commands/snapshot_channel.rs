use crate::db::update_snapshot_channel;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    prelude::Context,
};
#[command]
#[num_args(1)]
#[aliases(set_snapshot_channel, snap_channel, set_snap_channel)]
async fn snapshot_channel(ctx: &Context, msg: &Message, _: Args) -> CommandResult{

    
    
    let channel = msg.mention_channels[0].id;

    match update_snapshot_channel(&msg.guild_id.unwrap().as_u64(), &channel.as_u64()) {
        Ok(_) => msg.reply(ctx, format!("Successfully set channel to {}", channel.as_u64())).await?,
        Err(_) => msg.reply(ctx, "Could not set the snapshot channel, this is a fault with my code.").await?
    };

    Ok(())

}