use std::collections::HashSet;

use db::sea_orm::{Set, EntityTrait};
use serenity::{
    client::Context,
    model::{gateway::Ready, event::ResumedEvent}
};

use crate::{events::Handler, Connection};

impl Handler{
    pub async fn handle_ready(&self, ctx: Context, _: Ready) {

        let current_user = ctx.cache.current_user().await;
        let data = ctx.data.read().await;
        let con = data.get::<Connection>().unwrap();

        let currently_stored_guilds: HashSet<i64> = db::guild::Entity::find().all(con).await.expect("Could not get currently stored guilds")
            .iter().map(|g| g.guild_id).collect();
        db::guild::Entity::insert_many(current_user.guilds(&ctx.http).await.expect("Could not get current guilds.")
            .iter().filter(|g| !currently_stored_guilds.contains(&(*g.id.as_u64() as i64)))
            .map(|g| {
                db::guild::ActiveModel{
                    guild_id: Set(*g.id.as_u64() as i64),
                    is_compliant: Set(false),
                    ..Default::default()
                }
            })
        ).exec(con).await.expect("Could not insert into database or no guilds to insert.");
    
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
│  ┬─┐┌─┐┌─┐┌┬┐┬ ┬  ├────────────────────────────────────────────────┤
│  ├┬┘├┤ ├─┤ ││└┬┘  │ https://github.com/KingPEPSALT/chains-bot.git. │
│  ┴└─└─┘┴ ┴─┴┘ ┴ o ├────────────────────────────────────────────────┤ 
└───────────────────┴────────────────────────────────────────────────┘
        ");
    }
    pub async fn handle_resume(&self, _: Context, _: ResumedEvent){
        println!("Resumed.");
    }
}