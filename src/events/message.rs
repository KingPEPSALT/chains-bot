
use serenity::{
    model::{
        channel::{Message, EmbedFooter},
        id::ChannelId, 
        event::MessageUpdateEvent
    },
    client::{
        Context
    },
    utils::Colour, builder::CreateEmbedFooter
};
use crate::events::Handler;
use crate::MemberCache;

impl Handler {
    pub async fn handle_message(&self, ctx: Context, msg: Message){
        let data = ctx.data.read().await;

        let guild_id = *msg.guild_id.unwrap().as_u64() as i64;
        let author_id = *msg.author.id.as_u64() as i64;

        if let Some(Some(t)) = data.get::<MemberCache>().expect("No MemberCache HashMap in client data.")
        .get(&(guild_id, author_id)){
            let user_colour = msg.member(&ctx.http).await.unwrap().colour(ctx.cache).await.unwrap_or(Colour::from(0xFFFFFF as u32));
            let mut attachments = msg.attachments;
            ChannelId(*t as u64).send_message(&ctx.http, |m| {
                m.embed(|e|{
                    e.colour(user_colour) // nice blue colour
                        .title(format!("{}#{}",msg.author.name, msg.author.discriminator))
                        .thumbnail(
                            msg.author.avatar_url()
                            .unwrap_or("http://is5.mzstatic.com/image/thumb/Purple128/v4/bd/f2/33/bdf233b6-9cd2-8329-077e-acc120fce628/source/512x512bb.jpg".to_string())
                        )
                        .description(msg.content)
                        .footer(|f| {
                            f.text(format!("by ID: {} in <#{}>. Message ID: {} ", author_id, t, msg.id))
                        })
                        .timestamp(msg.timestamp);
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
    }
    pub async fn handle_message_update(&self, ctx: Context, _old: Option<Message>, _new: Option<Message>, event: MessageUpdateEvent){
        let data = ctx.data.read().await;
        let msg = event;
        let auth = msg.author.unwrap();

        let guild_id = *msg.guild_id.unwrap().as_u64() as i64;
        let author_id = *auth.id.as_u64() as i64;
        
        if let Some(Some(t)) = data.get::<MemberCache>().expect("No MemberCache HashMap in client data.")
        .get(&(guild_id, author_id)){
            ChannelId(*t as u64)
            .send_message(&ctx.http, |m| {
                m.embed(|e|{
                    e.colour(Colour::DARK_TEAL)
                    .title(format!("{}#{} ({})", auth.name, auth.discriminator, author_id))
                    .description(format!("Message edited to: {}", msg.content.unwrap()))
                })
            }).await.unwrap();
        }
    }

}