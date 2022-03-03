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

use crate::db::get_guild;
#[command] 
#[num_args(1)]
#[aliases(log_messages, log, snap, snap_messages, snapshot_messages)]
async fn snapshot(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult{

    let qty = args.single::<u64>().unwrap();

    let request = match get_guild(msg.guild_id.unwrap().as_u64()) {
        Ok(t) => t,
        Err(e) => {
            msg.reply(ctx, format!("Could not get guild from database | {} | This is an error within the code.", e.to_string())).await?;
            return Ok(());
        },
    }; 

    let channel = match request.snapshot_channel
    { 
        Some(t) => t,
        None => {
            msg.reply(ctx, "No snapshot channel is set for the server.").await?;
            return Ok(());
        }
    };

    msg.delete(&ctx.http).await?;

    let mut log_file = String::new();
    let messages = msg.channel_id.messages(&ctx.http, |retriever| retriever.limit(qty)).await?;
    let mut iter = messages.iter().rev();
    let mut true_qty = 0; 

    while let Some(message) = iter.next() {
        true_qty = true_qty + 1;
        log_file = format!(
            "{}\n[{}]\n[{}{} ({})] {}\n",
            log_file, 
            message.timestamp.to_string(), 
            message.author.name, 
            message.author.discriminator, 
            message.author.id.as_u64(), 
            message.content
        );
    };

    log_file = format!(
        "SNAPSHOT [REQUESTOR: {}{} ({})] [{} MESSAGES]\n\n{}", 
        msg.author.name, 
        msg.author.discriminator, 
        msg.author.id.as_u64(),
        log_file, 
        true_qty
    );

    if let Err(e) = ChannelId::from(channel)
        .send_files(
            &ctx.http, vec![(log_file.as_bytes(), "log_file.txt")],
            |m| m.content(format!("`Snapshot File - {}`", msg.timestamp.to_string()))
        ).await
    {
        msg.reply(ctx, e.to_string() + " | There was an error sending the log file.").await?;
    };   

    Ok(())
    
}

