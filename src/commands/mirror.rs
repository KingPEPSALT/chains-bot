use crate::{commands::parse_channel, Connection};
use db::sea_orm::{EntityTrait, Set, ActiveModelTrait};
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    prelude::Context,
};

#[command]
#[num_args(2)]
#[aliases(mirror_channel, mirrorChannel, mirrorchannel, mirror-channel, mc, mirror)]
async fn mirror(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    if !msg.member(ctx).await.unwrap().permissions(ctx).await.unwrap().administrator(){
        msg.reply(ctx, "You need administrator for this command.").await?;
        return Ok(());
    }
}