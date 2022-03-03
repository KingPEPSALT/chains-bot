use crate::db::update_mod_role;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    prelude::Context,
};

#[command]
#[num_args(1)]
#[aliases(set_moderation_role, moderation_role, set_mod_channel)]

async fn mod_role(ctx: &Context, msg: &Message, _: Args) -> CommandResult{
    
    if !msg.member(ctx).await.unwrap().permissions(ctx).await.unwrap().administrator(){
        msg.reply(ctx, "You need administrator for this command.").await?;
        return Ok(());
    }
    
    let role = msg.mention_roles[0];

    match update_mod_role(&msg.guild_id.unwrap().as_u64(), &role.as_u64()) {
        Ok(_) => msg.reply(ctx, format!("Successfully set moderation role to `<@&{}>`", role.as_u64())).await?,
        Err(_) => msg.reply(ctx, "Could not set the moderation role, this is a fault with my code.").await?
    };

    Ok(())

}