pub mod commands;
pub mod db;

use db::{delete_guild, cache_watched_members, cache_watch_channels};
//use db::clear_compliancies;
use dotenv;
use std::{sync::Arc, collections::{HashSet, HashMap}};

use commands::{ping::*, snapshot::*, snapshot_channel::*, mod_role::*, disclaimer::*, watch_channel::*, watch::*};

use serenity::{
    async_trait,
    model::{event::ResumedEvent, gateway::Ready, prelude::Guild, guild::GuildUnavailable, id::ChannelId, channel::Message},
    framework::{standard::macros::group, StandardFramework},
    http::Http,
    client::bridge::gateway::ShardManager,
    prelude::*, utils::Colour,
};
use crate::db::add_guild;

extern crate tokio;

pub struct WatchMemberHandler;

impl TypeMapKey for WatchMemberHandler{
    type Value = HashMap<u64, Vec<u64>>;
}

pub struct WatchChannelHandler;

impl TypeMapKey for WatchChannelHandler{
    type Value = HashMap<u64, u64>;
}
pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer{
    type Value = Arc<Mutex<ShardManager>>;
}

#[group]
#[commands(ping, snapshot, snapshot_channel, mod_role, disclaimer, watch, watch_channel)]
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
    async fn guild_delete(&self, _: Context, g: GuildUnavailable, _: Option<Guild>){
        if g.unavailable{
            delete_guild(g.id.as_u64()).expect(&format!("Could not delete guild from database {}", g.id.as_u64()));
        }
    }
    async fn message(&self, ctx: Context, msg: Message){
        let data = ctx.data.read().await;
        if let Some(t) = data.get::<WatchMemberHandler>().expect("No WatchMemberHandler in data.").get(&msg.guild_id.unwrap().as_u64()){
            if t.contains(msg.author.id.as_u64()){
                ChannelId(*data.get::<WatchChannelHandler>().expect("No WatchChannelHandler in data").get(
                    &msg.guild_id.unwrap().as_u64()
                ).unwrap())
                .send_message(&ctx.http, |m| {
                    m.embed(|e|{
                        e.colour(Colour::DARK_TEAL)
                        .title(format!("{}#{} ({})",msg.author.name, msg.author.discriminator, msg.author.id.as_u64()))
                        .description(msg.content)
                    })
                }
            ).await.unwrap();
        }
    }
}
async fn resume(&self, _: Context, _: ResumedEvent){
    println!("Resumed.");
}
}
//use db::{create_watched_members_table, create_guilds_table};

#[tokio::main]
async fn main() {
    
    //create_watched_members_table().expect("Could not create WatchedMembers table.");
    //create_guilds_table().expect("Could not create Guilds table.");
    
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
    let wch = cache_watch_channels().expect("Could not cache into WatchChannelHandler");
    let wmem = cache_watched_members().expect("Could not cache into WatchMemeberHandler");

    println!("\nLooking through WatchChannel cache.");
    for (k, v) in &wch{
        println!("{} - {}", k, v);
    }
    println!("\nLooking through WatchMember cache.");
    for (k, v) in &wmem{
        println!("{}", k);
        for tv in v{
            println!("  |-{}", tv);
        }
    }
    let mut client =
        Client::builder(&token)
            .framework(framework)
            .type_map_insert::<WatchChannelHandler>(wch)
            .type_map_insert::<WatchMemberHandler>(wmem)
            .event_handler(Handler)
            .await
            .expect("Could not create client");

    if let Err(e) = client.start().await {
        println!("Client error: {:?}", e);
    }
   
}