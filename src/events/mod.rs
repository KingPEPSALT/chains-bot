use serenity::{
    async_trait,
    model::{
        event::{
            ResumedEvent, 
            MessageUpdateEvent
        }, 
        gateway::Ready, 
        prelude::Guild, 
        guild::GuildUnavailable, 
        channel::Message
    },
    client::{
        EventHandler, 
        Context
    }
};
mod message;
mod client;
mod guild;
pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    
    async fn ready(&self, ctx: Context, event: Ready) {
        self.handle_ready(ctx, event).await;
    }
    async fn guild_create(&self, ctx: Context, g: Guild, is_new: bool){
        self.handle_guild_create(ctx, g, is_new).await;
    }
    async fn guild_delete(&self, ctx: Context, event: GuildUnavailable, g: Option<Guild>){
        self.handle_guild_delete(ctx, event, g).await;
    }

    async fn resume(&self, ctx: Context, event: ResumedEvent){
        self.handle_resume(ctx, event).await;
    }
    async fn message_update(&self, ctx: Context, _old: Option<Message>, _new: Option<Message>, event: MessageUpdateEvent){
        self.handle_message_update(ctx, _old, _new, event).await;
    }
    
    async fn message(&self, ctx: Context, msg: Message){
        self.handle_message(ctx, msg).await;
    }
}