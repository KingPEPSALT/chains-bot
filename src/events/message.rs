use std::borrow::{Borrow, BorrowMut};
use serenity::{
    model::{
        channel::Message,
        id::ChannelId, 
        event::{MessageUpdateEvent, MessageDeleteEvent}
    },
    client::{
        Context
    },
    utils::Colour
};
use serenity::model::channel::Attachment;
use crate::events::Handler;
use crate::MemberCache;
use crate::MirrorChannelCache;

impl Handler {
    pub async fn handle_message(&self, ctx: Context, msg: Message){
        let data = &ctx.data.read().await;

        let guild_id = *msg.guild_id.unwrap().as_u64() as i64;
        let author_id = *msg.author.id.as_u64() as i64;
        let current_channel_id = *msg.channel_id.as_u64() as i64;

        if let Some(Some(t)) = data.get::<MemberCache>().expect("No MemberCache HashMap in client data.")
        .get(&(guild_id, author_id)){
            let user_colour = msg.member(&ctx.http).await.unwrap().colour(&ctx.cache).await.unwrap_or(Colour::from(0xFFFFFF as u32));
            let mut attachments= msg.attachments.clone();
            ChannelId(*t as u64).send_message(&ctx.http, |m| {
                m.embed(|e|{
                    e.colour(user_colour) // nice blue colour
                        .title(format!("{}#{}",msg.author.name, msg.author.discriminator))
                        .thumbnail(
                            msg.author.avatar_url()
                            .unwrap_or("http://is5.mzstatic.com/image/thumb/Purple128/v4/bd/f2/33/bdf233b6-9cd2-8329-077e-acc120fce628/source/512x512bb.jpg".to_string())
                        )
                        .description(&msg.content)
                        .footer(|f| {
                            f.text(format!("by ID: {} in <#{}>. Message ID: {} ", author_id, t, msg.id))
                        })
                        .timestamp(&msg.timestamp);
                    if attachments.len() > 0{
                        e.image(attachments.pop().unwrap().url);
                    }
                    e
                })
            }).await.unwrap();
            while let Some(attachment) = attachments.pop(){
                ChannelId(*t as u64).send_message(&ctx.http, |m| {
                    m.embed(|e| {
                        e.colour(user_colour)
                            .image(attachment.url)
                            .footer(|f| {
                                f.text(format!("by ID: {} in <#{}>. Message ID: {} ", author_id, t, msg.id))
                            })
                            .timestamp(msg.timestamp)
                    })
                }).await.unwrap();
            }
        }

        if let Some(c_id) = data.get::<MirrorChannelCache>().expect("No Mirror Channel Cache")
        .get(&current_channel_id) {
            let channel_name = &ctx.cache.guild_channel(msg.channel_id).await.unwrap().name;
            let user_colour = msg.member(&ctx.http).await.unwrap().colour(&ctx.cache).await.unwrap_or(Colour::from(0xFFFFFF as u32));
            let mut attachments = msg.attachments;

            ChannelId(*c_id as u64).send_message(&ctx.http, |m| {
                m.embed(|e|{
                    e.colour(user_colour) // nice blue colour
                        .title(format!("{}#{} (from #{})",msg.author.name, msg.author.discriminator, channel_name))
                        .thumbnail(
                            msg.author.avatar_url()
                                .unwrap_or("http://is5.mzstatic.com/image/thumb/Purple128/v4/bd/f2/33/bdf233b6-9cd2-8329-077e-acc120fce628/source/512x512bb.jpg".to_string())
                        )
                        .description(&msg.content)
                        .footer(|f| {
                            f.text(format!("by ID: {} in <#{}>. Message ID: {} ", author_id, c_id, msg.id))
                        })
                        .timestamp(&msg.timestamp);
                    if attachments.len() > 0{
                        e.image(attachments.pop().unwrap().url);
                    }
                    e
                })
            }).await.unwrap();
            while let Some(attachment) = attachments.pop(){
                ChannelId(*c_id as u64).send_message(&ctx.http, |m| {
                    m.embed(|e| {
                        e.colour(user_colour)
                            .image(attachment.url)
                            .footer(|f| {
                                f.text(format!("by ID: {} in <#{}>. Message ID: {} ", author_id, c_id, msg.id))
                            })
                            .timestamp(msg.timestamp)
                    })
                }).await.unwrap();
            }
        }
    }
    pub async fn handle_message_update(&self, ctx: Context, _old: Option<Message>, _new: Option<Message>, event: MessageUpdateEvent){
        let data = ctx.data.read().await;
        let msg = event;
        let auth = &msg.author.unwrap();

        let guild_id = *msg.guild_id.unwrap().as_u64() as i64;
        let author_id = *auth.id.as_u64() as i64;
        
        if let Some(Some(t)) = data.get::<MemberCache>().expect("No MemberCache HashMap in client data.")
        .get(&(guild_id, author_id)){
            let user_colour = msg.guild_id.unwrap().member(&ctx.http, auth).await.unwrap().colour(ctx.cache).await.unwrap_or(Colour::from(0xFFFFFF as u32));

            ChannelId(*t as u64)
            .send_message(&ctx.http, |m| {
                m.embed(|e|{
                    e.colour(user_colour)
                    .title(format!("{}#{}", auth.name, auth.discriminator))
                    .thumbnail(
                        auth.avatar_url()
                        .unwrap_or("http://is5.mzstatic.com/image/thumb/Purple128/v4/bd/f2/33/bdf233b6-9cd2-8329-077e-acc120fce628/source/512x512bb.jpg".to_string())
                    )
                    .description(format!("Message edited to: {}", msg.content.unwrap()))
                    .footer(|f| {
                        f.text(format!("by ID: {} in <#{}>. Message ID: {} ", author_id, t, msg.id))
                    })
                    .timestamp(msg.timestamp.unwrap())
                })
            }).await.unwrap();
        }
    }

}