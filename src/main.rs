pub mod commands;
pub mod db;

//use db::clear_compliancies;
use dotenv;
use std::{sync::Arc, collections::HashSet};

use commands::{ping::*, snapshot::*, snapshot_channel::*, mod_role::*, disclaimer::*};

use serenity::{
    async_trait,
    model::{event::ResumedEvent, gateway::Ready, prelude::Guild},
    framework::{standard::macros::group, StandardFramework},
    http::Http,
    client::bridge::gateway::ShardManager,
    prelude::*,
};
use crate::db::add_guild;

extern crate tokio;

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer{
    type Value = Arc<Mutex<ShardManager>>;
}


#[group]
#[commands(ping, snapshot, snapshot_channel, mod_role, disclaimer)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected.", ready.user.name);
    }
    async fn guild_create(&self, _: Context, g: Guild, is_new: bool){
        if is_new{
            add_guild(g.id.as_u64()).expect(&format!("Could not add guild to database {}", g.id.as_u64()));
        }
    }
    async fn resume(&self, _: Context, _: ResumedEvent){
        println!("Resumed.");
    }
}

#[tokio::main]
async fn main() {

    //clear_compliancies().expect("Couldn't populate column");

    let token = dotenv::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    let http = Http::new_with_token(&token);

    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        },
        Err(e) => panic!("Could not get application info {:?}", e)
    };


    let framework = StandardFramework::new()
            .configure(|c| c.owners(owners).prefix(dotenv::var("DISCORD_PREFIX").unwrap()))
            .group(&GENERAL_GROUP);

    let mut client =
        Client::builder(&token).framework(framework).event_handler(Handler).await.expect("Could not create client");

    if let Err(e) = client.start().await {
        println!("Client error: {:?}", e);
    }
}