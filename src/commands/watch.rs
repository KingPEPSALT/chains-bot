
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    prelude::Context
};
#[command] 
#[min_args(1)]
#[max_args(2)]
#[aliases(monitor)]
async fn watch(ctx: &Context, msg: &Message, args: Args) -> CommandResult{
    Ok(())
}