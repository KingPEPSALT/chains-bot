use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::{channel::Message, id::ChannelId, prelude::MessageId},
    prelude::Context
};
use std::mem::swap;
use crate::db::{get_guild};
#[command] 
#[min_args(1)]
#[max_args(2)]
#[aliases(log_messages, log, snap, snap_messages, snapshot_messages, snip)]
async fn snapshot(ctx: &Context, msg: &Message, args: Args) -> CommandResult{
    
    let request = match get_guild(msg.guild_id.unwrap().as_u64()) 
    {
        Ok(guild_model) => guild_model,
        Err(e) => {
            msg.reply(ctx, format!("Could not get guild from database | {} | This is an error within the code.", e.to_string())).await?;
            return Ok(());
        },
    };
    
    if ! request.disclaimer_compliant {

        msg.reply(ctx, format!("A server admin must accept the `{}disclaimer`", dotenv::var("DISCORD_PREFIX").unwrap())).await?;
        return Ok(());
    }

    if ! (match request.mod_role
    {
        Some(id) => msg.author.has_role(ctx, msg.guild_id.unwrap(), id).await?,
        _ => false,
    } || msg.member(ctx).await.unwrap().permissions(ctx).await.unwrap().administrator())
    {
        msg.reply(ctx, "You don't have the required role!").await?;
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

    let messages: Vec<Message>;
    let arg_array = args.raw().collect::<Vec<&str>>();

    let mut pre_message: u64 = 0;
    let mut post_message: u64;
    if arg_array.len() == 2{
        // change messages into message id if they are links 
        let parse_helper = |n :usize| arg_array[n].parse::<u64>().unwrap_or_else(|_| arg_array[0].split("/").nth(6).unwrap().parse::<u64>().unwrap());
        [pre_message, post_message] =  [parse_helper(0), parse_helper(1)];
        // convulted code to orientate the messages in the correct order and include the message given in the snip 
        if MessageId(post_message).created_at() > MessageId(pre_message).created_at() {
            swap(&mut pre_message, &mut post_message)
        }
        post_message = *msg.channel_id.messages(ctx, |message_retriever|
            message_retriever
                .before(post_message).limit(1)
        ).await?[0].id.as_u64();
        messages = msg.channel_id.messages(ctx, |message_retriever| 
            message_retriever
                .after(post_message)
        ).await?;
    }else{
        messages = msg.channel_id.messages(ctx, |message_retriever| 
            message_retriever.limit(arg_array[0].parse::<u64>().unwrap())
        ).await?;
    }
    
    let mut snapshot_file = String::new();
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
        if &pre_message == message.id.as_u64(){
            break;
        }

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

