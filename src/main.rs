pub mod commands;
pub mod events;

use events::Handler;
use dotenv;
use std::{sync::Arc, collections::{HashSet, HashMap}};

// use db::sea_orm { DatabaseConnection, Database, ConnectOptions, DbErr };
// use db::sea_orm;
// use serenity::Error;
// use tokio::time::Instant;
use std::time::Duration;
use db::sea_orm::{ConnectOptions, DbErr, Set, Database, EntityTrait, DbConn};
use db::*;
use commands::{ping::*, snapshot::*, snapshot_channel::*, mod_role::*, disclaimer::*, watch::*};

use serenity::{
    framework::{standard::macros::group, StandardFramework},
    http::Http,
    client::bridge::gateway::ShardManager,
    prelude::*
};

extern crate tokio;

pub struct Connection;
impl TypeMapKey for Connection{
    type Value = DbConn;
}
pub struct MemberCache;

impl TypeMapKey for MemberCache{
    type Value = HashMap<(i64, i64), Option<i64>>;
}
pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer{
    type Value = Arc<Mutex<ShardManager>>;
}

#[group]
#[commands(ping, snapshot, snapshot_channel, mod_role, disclaimer, watch)]
struct General;



#[tokio::main]
async fn main() -> Result<(), DbErr> {
    
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
    let mut opt = ConnectOptions::new(dotenv::var("DATABASE_URL").unwrap());
    opt
        .max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true);
    let con =  Database::connect(opt).await?;

    let framework = StandardFramework::new()
            .configure(|c| c.owners(owners).prefix(dotenv::var("DISCORD_PREFIX").unwrap()))
            .group(&GENERAL_GROUP);
    let mut watched_members: HashMap<(i64, i64), Option<i64>> = HashMap::new();
    for member in db::member::Entity::find().all(&con).await?{
        watched_members.insert((member.guild_id, member.user_id), member.watch_channel_id);
    };

    // println!("\nLooking through WatchChannel cache.");
    // for (k, v) in &wch{
    //     println!("{} - {}", k, v);
    // }
    // println!("\nLooking through WatchMember cache.");
    // for (k, v) in &wmem{
    //     println!("{}", k);
    //     for i in 0..v.len()-1{
    //         println!("  ├─ {}", v[i]);
    //     }
    //     println!("  └─ {}", v[v.len()-1]);
    // }

    let mut client =
        Client::builder(&token)
            .framework(framework)
            .type_map_insert::<MemberCache>(watched_members)
            .type_map_insert::<Connection>(con)
            .event_handler(Handler)
            .await
            .expect("Could not create client");

    if let Err(e) = client.start().await {
        println!("Client error: {:?}", e);
    }

    Ok(())
}





    /* BENCHMARKING CODE*/
    /*
    let dummies = _test_entries(5000).await?;
    
    let now = Instant::now();
    let db =  Database::connect(opt).await?;
    let finished_connect = now.elapsed();
    
    let res = db::guild::Entity::insert_many(dummies).exec(&db).await?;
    let finished_insert = now.elapsed() - finished_connect;
    
    let guilds: Vec<guild::Model> = db::guild::Entity::find().all(&db).await?;
    let finished_read = now.elapsed() - finished_insert - finished_connect;
    for guild in guilds{
        print!("{} ", guild.guild_id);
    }
    println!("\nConnection to database took: {:?}", finished_connect);
    println!("Insert of {} records took: {:?}", res.last_insert_id+1, finished_insert);
    println!("Read of {} records took: {:?}", res.last_insert_id+1, finished_read);
    */

// Creates n dummy guilds to insert for benchmarking purposes
async fn _test_entries(n: u64) -> Result<Vec<guild::ActiveModel>, DbErr> {
    let mut x = Vec::new();
    let mut i = 0;
    while i < n  {
        let guild = db::guild::ActiveModel {
            guild_id: Set(i as i64),
            is_compliant: Set(true),
            ..Default::default() 
        };
        x.push(guild);
        i = i+1;
    }
    Ok(x)
}
