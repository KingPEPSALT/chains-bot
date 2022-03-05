
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    prelude::Context
};
use crate::{db::add_watched_member, WatchMemberHandler};
#[command] 
#[num_args(1)]
#[aliases(monitor)]
async fn watch(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult{
    let user = args.single::<u64>().unwrap();
    let mut data = ctx.data.write().await;
    match add_watched_member(&msg.guild_id.unwrap().as_u64(), &user, data.get_mut::<WatchMemberHandler>().unwrap()){
        Err(e) => {
            msg.reply(&ctx.http, e.to_string()).await?;
        },
        Ok(_) => {
            msg.delete(&ctx.http).await?;
        }
    };
    Ok(())
}