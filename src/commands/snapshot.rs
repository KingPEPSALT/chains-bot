use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::{channel::Message, id::ChannelId},
    prelude::Context,
};

use crate::db::{get_guild};
#[command] 
#[num_args(1)]
#[aliases(log_messages, log, snap, snap_messages, snapshot_messages)]
async fn snapshot(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult{

    let request = match get_guild(msg.guild_id.unwrap().as_u64()) 
    {
        Ok(guild_model) => guild_model,
        Err(e) => {
            msg.reply(ctx, format!("Could not get guild from database | {} | This is an error within the code.", e.to_string())).await?;
            return Ok(());
        },
    };

    if ! (match request.mod_role
    {
        Some(id) => msg.author.has_role(ctx, msg.guild_id.unwrap(), id).await?,
        _ => false,
    } || msg.member(ctx).await.unwrap().permissions(ctx).await.unwrap().administrator())
    {
        msg.reply(ctx, "You don't have the required role!").await?;
        return Ok(());
    }

    let input_qty = args.single::<u64>().unwrap();

    if ! request.disclaimer_compliant {

        msg.reply(ctx, format!("A server admin must accept the `{}disclaimer`", dotenv::var("DISCORD_PREFIX").unwrap())).await?;
        return Ok(());
    }

    
    let _channel_id = match request.snapshot_channel
    { 
        Some(id) => id,
        None => {
            msg.reply(ctx, "No snapshot channel is set for the server.").await?;
            return Ok(());
        }
    };

    msg.delete(ctx).await?;

    let mut snapshot_file = String::new();
    let messages = msg.channel_id.messages(ctx, |message_retriever| message_retriever.limit(input_qty)).await?;
    let mut message_iter = messages.iter().rev();
    let mut true_qty = 0; 

    while let Some(message) = message_iter.next() {
        true_qty = true_qty + 1;
        let mut attachments = String::new();

        message.attachments.iter().for_each(|a| attachments += &format!("[ATTACHMENT: {}]\n", a.url));
        
        snapshot_file = format!(
            "{}\n[{}]\n[{}#{} ({})] {}\n{}",
            snapshot_file,
            message.timestamp.to_string(), 
            message.author.name, 
            message.author.discriminator, 
            message.author.id.as_u64(), 
            message.content,
            attachments
        );
    };

    snapshot_file = format!(
        "SNAPSHOT [REQUESTOR: {}#{} ({})] [{} MESSAGES]\n\n{}", 
        msg.author.name, 
        msg.author.discriminator, 
        msg.author.id.as_u64(),
        true_qty,
        snapshot_file
    );

    if let Err(e) = ChannelId::from(_channel_id)
        .send_files(
            &ctx.http, 
            vec![(snapshot_file.as_bytes(), "log_file.txt")],
            |message| message.content(format!("`Snapshot File - {}`", msg.timestamp.to_string()))
        ).await
    {
        msg.reply(ctx, e.to_string() + " | There was an error sending the snapshot file.").await?;
    };   

    Ok(())
    
}

