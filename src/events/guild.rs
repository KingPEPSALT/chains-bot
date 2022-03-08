use serenity::{
    model::guild::{
        GuildUnavailable,
        Guild
    }, 
    client::Context
};

use crate::{events::Handler, db::{add_guild, delete_guild}};

impl Handler {
    pub async fn handle_guild_create(&self, _: Context, g: Guild, is_new: bool){
        if is_new{
            add_guild(g.id.as_u64()).expect(&format!("Could not add guild to database {}", g.id.as_u64()));
        }
    }
    pub async fn handle_guild_delete(&self, _: Context, g: GuildUnavailable, _: Option<Guild>){
        if g.unavailable{
            delete_guild(g.id.as_u64()).expect(&format!("Could not delete guild from database {}", g.id.as_u64()));
        }
    }
}