use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
pub async fn ping(ctx: &Context, msg: &Message, _args: Args) -> CommandResult{
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}