use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    prelude::Context,
    futures::StreamExt, utils::Colour
};

use crate::db::{get_guild, update_compliancy};

const DISSEMINATION: &str = "The use of the snapshot command requires logging of information which must be disseminated to clients of this bot and of which they must agree to. Chain will not locally but, instead, within the guild, log this information from the member: usernames, discriminators, client IDs and messages sent within this server. It will send this data in bytes that will form a file at some designated channel; this file is never stored anywhere but within the server. It will not do anything more with the information gathered and to repeat, it will not store it locally.";

#[command] 
#[num_args(0)]
#[aliases(dissemination)]
async fn disclaimer(ctx: &Context, msg: &Message, _: Args) -> CommandResult{
    let _guild_info = match get_guild(msg.guild_id.unwrap().as_u64()) 
    {
        Ok(g) => g,
        Err(e) => {
            msg.reply(ctx, format!("Could not get guild from database | {} | This is an error within the code.", e.to_string())).await?;
            return Ok(());
        },
    };
    if msg.member(ctx).await.unwrap().permissions(ctx).await.unwrap().administrator()
    {
        
        let disclaimer = msg.channel_id.send_message(&ctx.http, |message |
            {
            message.embed(|embed|
                { 
                embed.title("TERMS OF SERVICE DISSEMINATION")
                .colour(Colour::DARK_GREEN)
                .description(format!("{}\n\n\nReact with a :green_circle: to agree to these conditions or a :red_circle: opt out of this.", DISSEMINATION));
                return embed;
                })
        }).await?;
        disclaimer.react(ctx, '🟢').await?;
        disclaimer.react(ctx, '🔴').await?;
        let mut react_collector = disclaimer
            .await_reactions(&ctx)
            .timeout(tokio::time::Duration::from_secs(20))
            .filter(|e| e.emoji.as_data().as_str() == "🟢"||e.emoji.as_data().as_str() == "🔴")
            .author_id(msg.author.id).await;

        if let Some(e) = react_collector.next().await{
            update_compliancy(&msg.guild_id.unwrap().as_u64(), e.as_inner_ref().emoji.as_data().as_str()=="🟢")?;
        }
        disclaimer.delete(ctx).await?;
        msg.delete(ctx).await?;
    } else {
        msg.reply(ctx, format!("A server admin must use the command `{}disclaimer`", dotenv::var("DISCORD_PREFIX").unwrap())).await?;
    }
    Ok(())
}