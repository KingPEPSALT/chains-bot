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
    let str_arg = args.single::<String>().unwrap();
    let channel = match str_arg[2..str_arg.len()-1].parse::<u64>(){
        Ok(t) => t,
        Err(_) => {
            msg.reply(ctx, "That is not a valid channel.").await?;
            return Ok(());
        }
    };
    match update_snapshot_channel(&msg.guild_id.unwrap().as_u64(), &channel) {
        Ok(e) => msg.reply(ctx, format!("Successfully set channel to {}", str_arg)).await?,
        Err(e) => msg.reply(ctx, "Could not set the snapshot channel, this is a fault with my code.").await?
    };

    Ok(())
}