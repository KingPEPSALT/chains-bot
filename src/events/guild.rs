use db::sea_orm::{Set, ActiveModelTrait, EntityTrait, ModelTrait};
use serenity::{
    model::guild::{
        GuildUnavailable,
        Guild
    }, 
    client::Context
};

use crate::Connection;

use super::Handler;


impl Handler {
    pub async fn handle_guild_create(&self, ctx: Context, g: Guild, is_new: bool){
        if is_new{

            let guild_id = *g.id.as_u64() as i64;

            db::guild::ActiveModel{
                guild_id: Set(guild_id),
                is_compliant: Set(false),
                ..Default::default()
            }.insert(ctx.data.read().await.get::<Connection>().expect("Connection to database does not exist.")).await
                .expect(&format!("Could not add Guild {} to database.", guild_id));
        }
    }
    pub async fn handle_guild_delete(&self, ctx: Context, g: GuildUnavailable, _: Option<Guild>){
        if g.unavailable{

            let guild_id = *g.id.as_u64() as i64;
            let data = ctx.data.read().await;
            let con = data.get::<Connection>().expect("Connection to database does not exist.");

            db::guild::Entity::find_by_id(guild_id)
                .one(con).await
                    .expect(&format!("Could not find Guild {} in database.", guild_id)).unwrap()
                .delete(con).await
                    .expect(&format!("Could not delete Guild {} from the database.", guild_id));
        }
    }
}