use crate::db::update_logging_channel;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    prelude::Context,
};
#[command]
#[min_args(1)]
async fn setloggingchannel(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult{
    let str_arg = args.single::<String>().unwrap();
    let channel = match str_arg[2..str_arg.len()-1].parse::<u64>(){
        Ok(t) => t,
        Err(_) => {
            msg.reply(ctx, "That is not a valid channel.").await?;
            return Ok(());
        }
    };
    if let Err(e) = update_logging_channel(&msg.guild_id.unwrap().as_u64(), &channel) {
        msg.reply(ctx, e.to_string()).await?;
    };
    Ok(())
}