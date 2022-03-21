use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::utils::Colour;
use serenity::prelude::*;
use tokio::time::Instant;

#[command]
async fn ping(ctx: &Context, msg: &Message, _: Args) -> CommandResult{

    let pong = msg.reply(&ctx.http, "Pong!").await?;
    let latency = Instant::now();
    pong.delete(&ctx.http).await?;
    let elapsed = latency.elapsed();
    let self_colour = ctx.http.get_member(
        *msg.guild_id.unwrap().as_u64(),
        *ctx.cache.current_user_id().await.as_u64()).await.unwrap()
        .colour(&ctx.cache).await
        .unwrap_or(Colour::from(0xFFFFFF as u32));
    msg.channel(&ctx.cache).await.unwrap().id().send_message(&ctx.http, |m| {
        m.embed(|e|{
            e.colour(self_colour)
            .title("Pong!")
            .description(format!("Latency: {:?}ms", elapsed.as_millis()))
        })
    }).await?;
    Ok(())
    
}