use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::{channel::Message, id::{ChannelId, UserId}, prelude::MessageId},
    prelude::Context
};
use std::{mem::swap, collections::HashMap};
use crate::{commands::enforce_compliancy};
#[command] 
#[min_args(1)]
#[max_args(2)]
#[aliases(log_messages, log, snap, snap_messages, snapshot_messages, snip)]
async fn snapshot(ctx: &Context, msg: &Message, args: Args) -> CommandResult{
    
    let guild = *msg.guild_id.unwrap().as_u64() as i64;
    let (compliant, is_request) = enforce_compliancy(ctx, msg, guild).await;
    if !compliant || is_request.is_none(){
        return Ok(())
    };
    let request = is_request.unwrap();
    let channel_id = match request.snap_channel_id
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
        let parse_helper = |n :usize| arg_array[n].parse::<u64>().or_else(|_| arg_array[n].split("/").nth(6).unwrap().parse::<u64>());
        [pre_message, post_message] = match [parse_helper(0), parse_helper(1)] {
            [Ok(a), Ok(b)] => [a,b],
            _ => {
                msg.reply(ctx, "That isn't a valid message ID.").await?;
                return Ok(());
            }
        };
        // convulted code to orientate the messages in the correct order and include the message given in the snip 
        if MessageId(post_message).created_at() > MessageId(pre_message).created_at() {
            swap(&mut pre_message, &mut post_message)
        }
        post_message = match msg.channel_id.messages(ctx, |message_retriever|
            message_retriever
                .before(post_message).limit(1)
        ).await {
            Ok(t) => *t[0].id.as_u64(),
            _ => {
                msg.reply(ctx, "That message is not in this channel or could not be resolved.").await?;
                return Ok(());
            }
        };
        messages = match msg.channel_id.messages(ctx, |message_retriever| 
            message_retriever
                .after(post_message)
        ).await{
            Ok(t) => t,
            _ => {
                msg.reply(ctx, "That message is not in this channel or could not be resolved.").await?;
                return Ok(());
            }
        };
    }else{
        messages = msg.channel_id.messages(ctx, |message_retriever| 
            message_retriever.limit(arg_array[0].parse::<u64>().unwrap())
        ).await?;
    }
    
    let mut snapshot_file = String::new();
    let mut message_iter = messages.iter().rev();
    let mut true_qty = 0; 
    let mut nickname_map: HashMap<UserId, String> = HashMap::new();
    while let Some(message) = message_iter.next() {
        true_qty = true_qty + 1;
        let mut attachments = String::new();
        message.attachments.iter().for_each(|a| attachments += &format!("[ATTACHMENT: {}]\n", a.url));

        if !nickname_map.contains_key(&message.author.id){
            let nickname = match message.author.nick_in(&ctx.http, request.guild_id as u64).await {
                Some(t) => format!("({})", t),
                None => "".into()
            };
            nickname_map.insert(message.author.id, nickname);
        }


        snapshot_file = format!(
            "{}\n[{}]\n[{}{}#{} ({})] {}\n{}",
            snapshot_file,
            message.timestamp.to_string(), 
            message.author.name, 
            nickname_map.get(&message.author.id).unwrap(),
            message.author.discriminator, 
            message.author.id.as_u64(), 
            message.content,
            attachments
        );
        if &pre_message == message.id.as_u64(){
            break;
        }

    };

    let requester_tag = msg.author.nick_in(ctx, guild  as u64).await.unwrap_or(" ".to_string());
    snapshot_file = format!(
        "SNAPSHOT [REQUESTOR: {}({})#{} ({})] [{} MESSAGES]\n\n{}", 
        msg.author.name, 
        requester_tag,
        msg.author.discriminator, 
        msg.author.id.as_u64(),
        true_qty,
        snapshot_file
    );

    if let Err(e) = ChannelId::from(channel_id as u64)
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

