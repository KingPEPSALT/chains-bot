use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
async fn ping(ctx: &Context, msg: &Message, _: Args) -> CommandResult{

    msg.reply(ctx, "Pong!").await?;
    
    Ok(())
    
}