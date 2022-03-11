
// use serenity::{
//     model::{
//         channel::Message,
//         id::ChannelId, 
//         event::MessageUpdateEvent
//     },
//     client::{
//         Context
//     },
//     utils::Colour
// };
// use crate::events::Handler;
// use crate::{WatchMemberHandler, WatchChannelHandler};

// impl Handler {
//     pub async fn handle_message(&self, ctx: Context, msg: Message){
//         let data = ctx.data.read().await;
//         if let Some(t) = data.get::<WatchMemberHandler>().expect("No WatchMemberHandler in data.").get(&msg.guild_id.unwrap().as_u64()){
//             if t.contains(msg.author.id.as_u64()){
//                 ChannelId(*data.get::<WatchChannelHandler>().expect("No WatchChannelHandler in data").get(
//                     &msg.guild_id.unwrap().as_u64()
//                 ).unwrap())
//                 .send_message(&ctx.http, |m| {
//                     m.embed(|e|{
//                         e.colour(Colour::DARK_TEAL)
//                         .title(format!("{}#{} ({})",msg.author.name, msg.author.discriminator, msg.author.id.as_u64()))
//                         .description(msg.content)
//                     })
//                 }).await.unwrap();
//             }
//         }
//     }
//     pub async fn handle_message_update(&self, ctx: Context, _old: Option<Message>, _new: Option<Message>, event: MessageUpdateEvent){
//         let data = ctx.data.read().await;
//         let msg = event;
//         let auth = msg.author.unwrap();
//         if let Some(t) = data.get::<WatchMemberHandler>().expect("No WatchMemberHandler in data.").get(&msg.guild_id.unwrap().as_u64()){
//             if t.contains(auth.id.as_u64()){
//                 ChannelId(*data.get::<WatchChannelHandler>().expect("No WatchChannelHandler in data").get(
//                     &msg.guild_id.unwrap().as_u64()
//                 ).unwrap())
//                 .send_message(&ctx.http, |m| {
//                     m.embed(|e|{
//                         e.colour(Colour::DARK_TEAL)
//                         .title(format!("{}#{} ({})", auth.name, auth.discriminator, auth.id.as_u64()))
//                         .description(format!("New message: {}", msg.content.unwrap()))
//                     })
//                 }).await.unwrap();
//             }
//         }
//     }

// }