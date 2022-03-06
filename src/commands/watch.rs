
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    prelude::Context
};
use crate::{db::{add_watched_member, remove_watched_member}, WatchMemberHandler};
#[command] 
#[num_args(1)]
#[aliases(monitor)]
async fn watch(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult{
    let user = args.single::<u64>().unwrap();
    let mut data = ctx.data.write().await;
    let cache = data.get_mut::<WatchMemberHandler>().unwrap();
    let guild = *msg.guild_id.unwrap().as_u64();
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