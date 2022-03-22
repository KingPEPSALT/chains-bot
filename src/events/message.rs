use crate::events::Handler;
use crate::MemberCache;
use crate::MirrorChannelCache;
use serenity::{
    client::Context,
    model::{channel::Message, event::MessageUpdateEvent, id::ChannelId},
    utils::Colour,
};
use crate::commands::mirror_command;

impl Handler {
    pub async fn handle_message(&self, ctx: Context, msg: Message) {
        let data = &ctx.data.read().await;

        let guild_id = *msg.guild_id.unwrap().as_u64() as i64;
        let author_id = *msg.author.id.as_u64() as i64;
        let current_channel_id = *msg.channel_id.as_u64() as i64;

        if let Some(Some(t)) = data
            .get::<MemberCache>()
            .expect("No MemberCache HashMap in client data.")
            .get(&(guild_id, author_id))
        {
            let user_colour = msg
                .member(&ctx.http)
                .await
                .unwrap()
                .colour(&ctx.cache)
                .await
                .unwrap_or(Colour::from(0xFFFFFF as u32));
            let mut attachments = msg.attachments.iter();

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
                        e.image(&attachments.next().unwrap().url);
                    }
                    e
                })
            }).await.unwrap();
            while let Some(attachment) = attachments.next() {
                ChannelId(*t as u64)
                    .send_message(&ctx.http, |m| {
                        m.embed(|e| {
                            e.colour(user_colour)
                                .image(&attachment.url)
                                .footer(|f| {
                                    f.text(format!(
                                        "by ID: {} in <#{}>. Message ID: {} ",
                                        author_id, t, msg.id
                                    ))
                                })
                                .timestamp(msg.timestamp)
                        })
                    })
                    .await
                    .unwrap();
            }
        }

        mirror_command::events::handle_message(&ctx, &msg).await

    }
    pub async fn handle_message_update(
        &self,
        ctx: Context,
        old: Option<Message>,
        new: Option<Message>,
        event: MessageUpdateEvent,
    ) {
        let data = ctx.data.read().await;
        let msg = &event;
        let auth = msg.author.as_ref().unwrap();

        let guild_id = *msg.guild_id.unwrap().as_u64() as i64;
        let author_id = *auth.id.as_u64() as i64;

        if let Some(Some(t)) = data
            .get::<MemberCache>()
            .expect("No MemberCache HashMap in client data.")
            .get(&(guild_id, author_id))
        {
            let user_colour = msg
                .guild_id
                .unwrap()
                .member(&ctx.http, auth)
                .await
                .unwrap()
                .colour(&ctx.cache)
                .await
                .unwrap_or(Colour::from(0xFFFFFF as u32));

            ChannelId(*t as u64)
            .send_message(&ctx.http, |m| {
                m.embed(|e|{
                    e.colour(user_colour)
                    .title(format!("{}#{}", auth.name, auth.discriminator))
                    .thumbnail(
                        auth.avatar_url()
                        .unwrap_or("http://is5.mzstatic.com/image/thumb/Purple128/v4/bd/f2/33/bdf233b6-9cd2-8329-077e-acc120fce628/source/512x512bb.jpg".to_string())
                    )
                    .description(format!("Message edited to: {}", msg.content.as_ref().unwrap()))
                    .footer(|f| {
                        f.text(format!("by ID: {} in <#{}>. Message ID: {} ", author_id, t, msg.id))
                    })
                    .timestamp(msg.timestamp.unwrap())
                })
            }).await.unwrap();
        }

        mirror_command::events::handle_message_update(&ctx, old, new, &event).await;
    }
}
