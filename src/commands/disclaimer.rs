use db::sea_orm::{EntityTrait, Set, ActiveModelTrait};
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    prelude::Context,
    futures::StreamExt, utils::Colour
};

use crate::Connection;

const DISSEMINATION: &str = "The use of this bot requires logging to an external database for the bot to query. It is used only to store configuration for a guild and will store user IDs, guild IDs, roleIDs and channel IDs only. Chains will do no more than utilise this information in the commands available and this information will never leave the database otherwise.";

#[command] 
#[num_args(0)]
#[aliases(dissemination)]
async fn disclaimer(ctx: &Context, msg: &Message, _: Args) -> CommandResult{

    let guild_id = *msg.guild_id.unwrap().as_u64();
    let data = ctx.data.read().await;
    let con = data.get::<Connection>().unwrap();

    let mut guild: db::guild::ActiveModel = match db::guild::Entity::find_by_id(guild_id as i64).one(con).await
    {
        Ok(Some(g)) => g.into(),
        _ => {
            msg.reply(ctx, "Could not get guild from database | This is an error within the code.").await?;
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
            guild.is_compliant = Set(e.as_inner_ref().emoji.as_data().as_str()=="🟢");
            guild.update(con).await?;
        }
        disclaimer.delete(ctx).await?;
        msg.delete(ctx).await?;
    } else {
        msg.reply(ctx, format!("A server admin must use the command `{}disclaimer`", dotenv::var("DISCORD_PREFIX").unwrap())).await?;
    }
    Ok(())
}