use std::{sync::Arc, env, collections::HashSet};

use commands::{ping::*};

use serenity::{
    async_trait,
    model::{event::ResumedEvent, gateway::Ready},
    framework::{standard::macros::group, StandardFramework},
    http::Http,
    client::bridge::gateway::ShardManager,
    prelude::*,
};

use tracing::{info};
extern crate tokio;

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer{
    type Value = Arc<Mutex<ShardManager>>;
}


mod commands;

#[group]
#[commands(ping)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {

    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected.", ready.user.name);
    }

    async fn resume(&self, _: Context, _: ResumedEvent){
        info!("Resumed.");
    }
}

#[tokio::main]
async fn main() {

    tracing_subscriber::fmt::init();

    let token = env::var("DISCORD_TOKEN")
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
            .configure(|c| c.owners(owners).prefix("-"))
            .group(&GENERAL_GROUP);

    let mut client =
        Client::builder(&token).framework(framework).event_handler(Handler).await.expect("Could not create client");

    if let Err(e) = client.start().await {
        println!("Client error: {:?}", e);
    }
}