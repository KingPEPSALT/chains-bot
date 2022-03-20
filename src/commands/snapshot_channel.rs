use crate::{commands::parse_channel, Connection};
use db::sea_orm::{EntityTrait, Set, ActiveModelTrait};
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

    let channel_id = match parse_channel(args.single::<String>().unwrap()){
        Ok(id) => id,
        Err(_) => {
            msg.reply(ctx, "That is not a valid channel ID or channel mention.").await?;
            return Ok(());
        }
    } as i64;
    
    let data = ctx.data.read().await;
    let con = data.get::<Connection>().unwrap();

    let mut guild : db::guild::ActiveModel = db::guild::Entity::find_by_id(*msg.guild_id.unwrap().as_u64() as i64).one(con).await?.unwrap().into();
    guild.snap_channel_id = Set(Some(channel_id));
    match guild.update(con).await {
        Ok(_) => msg.reply(ctx, format!("Successfully set channel to `<#{}>`", channel_id)).await?,
        Err(_) => msg.reply(ctx, "Could not set the snapshot channel, this is a fault with my code.").await?
    };

    Ok(())

}