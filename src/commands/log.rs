use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::{channel::Message, id::ChannelId},
    prelude::Context,
    futures::StreamExt
};
use std::{
    fs::File,
    io::Write,
    env
};
use crate::db;
#[command] FUCKKKKKK FUCK XD LOLE 
#[min_args(1)]
async fn log(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult{
    let qty = args.single::<u64>().unwrap();
    let request = match db::get_guild(msg.guild_id.unwrap().as_u64()) {
        Ok(t) => t,
        Err(e) => {
            msg.reply(ctx, e.to_string()).await?;
            return Ok(());
        },
    }; 
    let channel = match request.logging_channel_id
    { 
        Some(t) => t,
        None => {
            msg.reply(ctx, "No logging channel is set for the server.").await?;
            return Ok(());
        }
    };
    let mut log_file = String::new();
    log_file = format!("{lf}\nLOG_FILE [{name}{disc} ({id})] [{q} MESSAGES]", lf = log_file, q = qty, name = msg.author.name, disc = msg.author.discriminator, id = msg.author.id.as_u64());
    msg.delete(&ctx.http).await?;

    let messages = msg.channel_id.messages(&ctx.http, |retriever| retriever.limit(qty)).await?;
    let mut iter = messages.iter().rev();
    
    while let Some(message) = iter.next() {
        log_file = format!("{lf}\n[{name}{disc} ({id})]", lf = log_file, name = message.author.name, disc = message.author.discriminator, id = message.author.id.as_u64());
        log_file = format!("{lf}\n[{time}] {content}\n", lf = log_file, time = message.timestamp.to_string(), content = message.content);
    };
    
    if let Err(e) = ChannelId::from(channel).send_files(&ctx.http, vec![(log_file.as_bytes(), "log_file.txt")], |m| m.content(format!("`Logging File - {}`", msg.timestamp.to_string()))).await{
        msg.reply(ctx, e.to_string() + "| There was an error sending the log file.").await?;
    };      
    Ok(())
    
}

