use serenity::{
    async_trait,
    model::{event::{ResumedEvent, MessageUpdateEvent}, gateway::Ready, prelude::Guild, guild::GuildUnavailable, id::ChannelId, channel::Message},
    utils::Colour, client::{EventHandler, Context}
};
use crate::{db::{delete_guild, add_guild}, WatchMemberHandler, WatchChannelHandler};
pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    
    async fn ready(&self, _: Context, _: Ready) {
        println!("
┌────────────────────────────────────────────────────────────────────┐
│ ▄████████    ▄█    █▄       ▄████████  ▄█  ███▄▄▄▄      ▄████████  │
│ ███    ███   ███    ███     ███    ███ ███  ███▀▀▀██▄   ███    ███ │
│ ███    █▀    ███    ███     ███    ███ ███▌ ███   ███   ███    █▀  │
│ ███         ▄███▄▄▄▄███▄▄   ███    ███ ███▌ ███   ███   ███        │
│ ███        ▀▀███▀▀▀▀███▀  ▀███████████ ███▌ ███   ███ ▀███████████ │
│ ███    █▄    ███    ███     ███    ███ ███  ███   ███          ███ │
│ ███    ███   ███    ███     ███    ███ ███  ███   ███    ▄█    ███ │
│ ████████▀    ███    █▀      ███    █▀  █▀    ▀█   █▀   ▄████████▀  │
├───────────────────┬────────────────────────────────────────────────┤
│  ┬─┐┌─┐┌─┐┌┬┐┬ ┬  ├ pepsalt#1662|Salivala#1787|Anthony Fuller#1767 ┤
│  ├┬┘├┤ ├─┤ ││└┬┘  ├ https://github.com/KingPEPSALT/chains-bot.git. ┤
│  ┴└─└─┘┴ ┴─┴┘ ┴ o ├─────────────── <3 have fun! <3 ────────────────┤ 
└───────────────────┴────────────────────────────────────────────────┘
");
    }
    async fn guild_create(&self, _: Context, g: Guild, is_new: bool){
        if is_new{
            add_guild(g.id.as_u64()).expect(&format!("Could not add guild to database {}", g.id.as_u64()));
        }
    }
    async fn guild_delete(&self, _: Context, g: GuildUnavailable, _: Option<Guild>){
        if g.unavailable{
            delete_guild(g.id.as_u64()).expect(&format!("Could not delete guild from database {}", g.id.as_u64()));
        }
    }
    async fn message(&self, ctx: Context, msg: Message){
        let data = ctx.data.read().await;
        if let Some(t) = data.get::<WatchMemberHandler>().expect("No WatchMemberHandler in data.").get(&msg.guild_id.unwrap().as_u64()){
            if t.contains(msg.author.id.as_u64()){
                ChannelId(*data.get::<WatchChannelHandler>().expect("No WatchChannelHandler in data").get(
                    &msg.guild_id.unwrap().as_u64()
                ).unwrap())
                .send_message(&ctx.http, |m| {
                    m.embed(|e|{
                        e.colour(Colour::DARK_TEAL)
                        .title(format!("{}#{} ({})",msg.author.name, msg.author.discriminator, msg.author.id.as_u64()))
                        .description(msg.content)
                    })
                }).await.unwrap();
            }
        }
    }
    async fn resume(&self, _: Context, _: ResumedEvent){
        println!("Resumed.");
    }
    async fn message_update(&self, ctx: Context, _old: Option<Message>, _new: Option<Message>, event: MessageUpdateEvent){
        let data = ctx.data.read().await;
        let msg = event;
        let auth = msg.author.unwrap();
        if let Some(t) = data.get::<WatchMemberHandler>().expect("No WatchMemberHandler in data.").get(&msg.guild_id.unwrap().as_u64()){
            if t.contains(auth.id.as_u64()){
                ChannelId(*data.get::<WatchChannelHandler>().expect("No WatchChannelHandler in data").get(
                    &msg.guild_id.unwrap().as_u64()
                ).unwrap())
                .send_message(&ctx.http, |m| {
                    m.embed(|e|{
                        e.colour(Colour::DARK_TEAL)
                        .title(format!("{}#{} ({})", auth.name, auth.discriminator, auth.id.as_u64()))
                        .description(format!("New message: {}", msg.content.unwrap()))
                    })
                }).await.unwrap();
            }
        }
    }

}