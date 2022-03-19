use crate::Connection;
use db::sea_orm::{EntityTrait, Set, ActiveModelTrait};
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    prelude::Context,
};

#[command]
#[num_args(1)]
#[aliases(set_moderation_role, moderation_role, set_mod_channel)]

async fn mod_role(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult{
    
    if !msg.member(ctx).await.unwrap().permissions(ctx).await.unwrap().administrator(){
        msg.reply(ctx, "You need administrator for this command.").await?;
        return Ok(());
    }
    
    let moderation_role_id = match args.single::<u64>() {
        Ok(t) => t,
        Err(_) => {
            if msg.mention_roles.len() == 0{
                msg.reply(ctx, "You did not supply a sufficient role ID or mention a role.").await?;
                return Ok(())
            }
            *msg.mention_roles[0].as_u64()
        }
    } as i64;

    let data = ctx.data.read().await;
    let con = data.get::<Connection>().unwrap();
    let mut guild : db::guild::ActiveModel = db::guild::Entity::find_by_id(*msg.guild_id.unwrap().as_u64() as i64).one(con).await?.unwrap().into();
    guild.moderation_role_id = Set(Some(moderation_role_id));
    
    match guild.update(con).await {
        Ok(_) => msg.reply(ctx, format!("Successfully set moderation role to `<@&{}>`", moderation_role_id)).await?,
        Err(_) => msg.reply(ctx, "Could not set the moderation role, this is a fault with my code.").await?
    };

    Ok(())

}