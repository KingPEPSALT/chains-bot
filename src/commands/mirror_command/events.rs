use serenity::{
    client::Context,
    model::{channel::Message, event::MessageUpdateEvent, id::ChannelId},
    utils::Colour,
};
use crate::events::Handler;

use crate::MirrorChannelCache;

pub async fn handle_message(ctx: & Context, msg: & Message) {
    let data = &ctx.data.read().await;
    let current_channel_id = *msg.channel_id.as_u64() as i64;
    let author_id = *msg.author.id.as_u64() as i64;
    if let Some(c_id) = data
        .get::<MirrorChannelCache>()
        .expect("No Mirror Channel Cache")
        .get(&current_channel_id)
    {
        let channel_name = &ctx.cache.guild_channel(msg.channel_id).await.unwrap().name;
        let user_colour = msg
            .member(&ctx.http)
            .await
            .unwrap()
            .colour(&ctx.cache)
            .await
            .unwrap_or(Colour::from(0xFFFFFF as u32));
        let mut attachment_iter = msg.attachments.iter();

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
                if attachment_iter.len() > 0{
                    e.image(&attachment_iter.next().unwrap().url);
                }
                e
            })
        }).await.unwrap();
        while let Some(attachment) = attachment_iter.next() {
            ChannelId(*c_id as u64)
                .send_message(&ctx.http, |m| {
                    m.embed(|e| {
                        e.colour(user_colour)
                            .image(&attachment.url)
                            .footer(|f| {
                                f.text(format!(
                                    "by ID: {} in <#{}>. Message ID: {} ",
                                    author_id, c_id, msg.id
                                ))
                            })
                            .timestamp(msg.timestamp)
                    })
                })
                .await
                .unwrap();
        }
    }
}

pub async fn handle_message_update(
    ctx: &Context,
    old: Option<Message>,
    new: Option<Message>,
    event: & MessageUpdateEvent,
) {
    let data = ctx.data.read().await;
    let msg = event;
    let auth = msg.author.as_ref().unwrap();

    let guild_id = *msg.guild_id.unwrap().as_u64() as i64;
    let author_id = *auth.id.as_u64() as i64;
    let message_channel_id = &(*msg.channel_id.as_u64() as i64);

    if let Some(mirror_channel_id) = data.get::<MirrorChannelCache>()
        .expect("No Mirror cache")
        .get(message_channel_id) {
            match (old, new, msg.guild_id) {
                (Some(old_message), Some(new_message), Some(guild_id)) => {
                    let user_colour = guild_id.member(&ctx.http, auth)
                        .await.unwrap().colour(&ctx.cache).await.unwrap_or(Colour::from(0xFFFFFF as u32));
                    ChannelId(*mirror_channel_id as u64).send_message(&ctx.http, |m| {
                        m.embed(|e|{
                            e.colour(user_colour)
                            .title("test")
                                .description("description")
                        })
                    }).await.unwrap();
                }
                (None, None, None ) => {println!("ok1")}
                (None, None, Some(x)) => {println!("ok2")}
                (_,_,_) => { println!("ok") }
                
        }
    }
}