
use serenity::{
    model::{
        channel::Message,
        id::ChannelId, 
        event::MessageUpdateEvent
    },
    client::{
        Context
    },
    utils::Colour
};
use crate::events::Handler;
use crate::MemberCache;
use crate::MirrorChannelCache;

impl Handler {
    pub async fn handle_message(&self, ctx: Context, msg: Message){
        let data = ctx.data.read().await;

        let guild_id = *msg.guild_id.unwrap().as_u64() as i64;
        let author_id = *msg.author.id.as_u64() as i64;
        let current_channel_id = *msg.channel_id.as_u64() as i64;

        if let Some(Some(t)) = data.get::<MemberCache>().expect("No MemberCache HashMap in client data.")
        .get(&(guild_id, author_id)){
            ChannelId(*t as u64).send_message(&ctx.http, |m| {
                m.embed(|e|{
                    e.colour(Colour::DARK_TEAL)
                    .title(format!("{}#{} ({})",msg.author.name, msg.author.discriminator, author_id))
                    .description(&msg.content)
                })
            }).await.unwrap();
        }

        if let Some(c_id) = data.get::<MirrorChannelCache>().expect("No Mirror Channel Cache")
        .get(&current_channel_id) {
            let channel_name = &ctx.cache.guild_channel(msg.channel_id).await.unwrap().name;
            ChannelId(*c_id as u64).send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.color(Colour::RED)
                    .title(format!("{}#{} Origin Chan({})", msg.author.name, msg.author.discriminator, channel_name))
                    .description(&msg.content)
                })
            }).await.unwrap();
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
                    .description(format!("New message: {}", msg.content.unwrap()))
                })
            }).await.unwrap();
        }
    }

}