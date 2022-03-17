use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::{channel::Message, id::{ChannelId, UserId}, prelude::MessageId},
    prelude::Context
};
use std::{mem::swap, collections::HashMap};
use crate::commands::{enforce_compliancy, is_moderator};
#[command] 
#[min_args(1)]
#[max_args(2)]
#[aliases(log_messages, log, snap, snap_messages, snapshot_messages, snip)]
async fn snapshot(ctx: &Context, msg: &Message, args: Args) -> CommandResult{
    
    if !is_moderator(ctx, msg).await{
        msg.reply(ctx, "You must be a moderator to run this command.").await?;
        return Ok(())
    }

    // get the guild id and cast to i64 so it can be used with the database
    let guild = *msg.guild_id.unwrap().as_u64();

    // check that the guild is compliant with the bot disclaimer
    // see mod.rs for enforce_compliancy
    let (compliant, possible_request) = enforce_compliancy(ctx, msg).await;
    if !compliant || possible_request.is_none(){
        return Ok(())
    };

    // get the channel to send the snap to from the sea_orm model
    let channel_id = match possible_request.unwrap().snap_channel_id
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

    // we must initialise this as it will be used later for iteration.
    let mut first_message: u64 = 0;
    let mut last_message: u64;

    // check if two args are given, if so, the arguments must be 2 message links/ids.
    if arg_array.len() == 2{

        // change messages into message id if they are links.
        let parse_helper = |n :usize| arg_array[n].parse::<u64>().or_else(|_| arg_array[n].split("/").nth(6).unwrap().parse::<u64>());
        [first_message, last_message] = match [parse_helper(0), parse_helper(1)] {
            [Ok(a), Ok(b)] => [a,b],
            _ => {
                msg.reply(ctx, "That isn't a valid message ID.").await?;
                return Ok(());
            }
        };

        // chronologically order the messages.
        if MessageId(last_message).created_at() > MessageId(first_message).created_at() {
            swap(&mut first_message, &mut last_message)
        }

        // to include the last_message within the snap, we must take the message before it chronologically.
        last_message = match msg.channel_id.messages(ctx, |message_retriever|
            message_retriever
                .before(last_message).limit(1)
        ).await {
            Ok(t) => *t[0].id.as_u64(),
            _ => {
                msg.reply(ctx, "That message is not in this channel or could not be resolved.").await?;
                return Ok(());
            }
        };
        // then we will get a list of messages after (and including) the first_message given as an argument  
        messages = match msg.channel_id.messages(ctx, |message_retriever| 
            message_retriever
                .after(last_message)
        ).await{
            Ok(t) => t,
            _ => {
                msg.reply(ctx, "That message is not in this channel or could not be resolved.").await?;
                return Ok(());
            }
        };
    
    }else{
        // no message ids were given so we simply need to snap the last n messages in the channel
        messages = msg.channel_id.messages(ctx, |message_retriever| 
            message_retriever.limit(arg_array[0].parse::<u64>().unwrap())
        ).await?;
    }

    let mut snapshot_file = String::new(); // the messages will be stored here which will be sent to discord as a file from bytes
    let mut message_iter = messages.iter().rev(); // iterate over the messages, in reverse to display them correctly.
    let mut true_qty = 0;
    
    // to get the nickname of a user in a server, you must query the discord API however this is slow and can take many seconds for large message quantities
    // to speed this up, a hashmap is created with a 
    let mut nickname_map: HashMap<UserId, String> = HashMap::new(); 
    
    // we will iterate through the messages until we read the last message
    while let Some(message) = message_iter.next() {
        true_qty = true_qty + 1;
        let mut attachments = String::new();
        message.attachments.iter().for_each(|a| attachments += &format!("[ATTACHMENT: {}]\n", a.url));

        // try to get the nickname from the hashmap and if it isn't there, take it from the discord API and insert it into the hashmap
        let nickname: String = match nickname_map.get(&message.author.id){
            Some(t) => t.into(),
            None => {
                let n = &match message.author.nick_in(&ctx.http, guild).await {
                    Some(t) => format!("({})", t),
                    None => "".into()
                };
                nickname_map.insert(message.author.id, n.into());
                n.into()
            }
        };

        snapshot_file = format!(
            "{}\n[{}]\n[{}{}#{} ({})] {}\n{}",
            snapshot_file,
            message.timestamp.to_string(), 
            message.author.name, 
            nickname,
            message.author.discriminator, 
            message.author.id.as_u64(), 
            message.content,
            attachments
        );
        if &first_message == message.id.as_u64(){
            break;
        }

    };

    // file heading
    let requester_tag = msg.author.nick_in(ctx, guild as u64).await.unwrap_or(" ".to_string());
    snapshot_file = format!(
        "SNAPSHOT [REQUESTOR: {}{}#{} ({})] [{} MESSAGES]\n\n{}", 
        msg.author.name, 
        requester_tag,
        msg.author.discriminator, 
        msg.author.id.as_u64(),
        true_qty,
        snapshot_file
    );

    // try to send the string as a file
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

