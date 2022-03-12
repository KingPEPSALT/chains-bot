// pub mod commands;
// pub mod db;
// pub mod events;

// use db::{cache_watched_members, cache_watch_channels};
// use events::Handler;
// use dotenv;
// use std::{sync::Arc, collections::{HashSet, HashMap}};

// use commands::{ping::*, snapshot::*, snapshot_channel::*, mod_role::*, disclaimer::*, watch_channel::*, watch::*};

// use serenity::{
//     framework::{standard::macros::group, StandardFramework},
//     http::Http,
//     client::bridge::gateway::ShardManager,
//     prelude::*
// };

// extern crate tokio;

// pub struct WatchMemberHandler;

// impl TypeMapKey for WatchMemberHandler{
//     type Value = HashMap<u64, Vec<u64>>;
// }

// pub struct WatchChannelHandler;

// impl TypeMapKey for WatchChannelHandler{
//     type Value = HashMap<u64, u64>;
// }
// pub struct ShardManagerContainer;

// impl TypeMapKey for ShardManagerContainer{
//     type Value = Arc<Mutex<ShardManager>>;
// }

// #[group]
// #[commands(ping, snapshot, snapshot_channel, mod_role, disclaimer, watch, watch_channel)]
// struct General;



// use db::{create_watched_members_table, create_guilds_table};
// #[tokio::main]
// async fn main() {
    
//     println!();
//     match create_watched_members_table(){
//         Err(_) => println!("WatchedMembers table likely exists."),
//         _ => println!("WatchedMembers table created.")
//     }    
//     match create_guilds_table(){
//         Err(_) => println!("Guilds table likely exists."),
//         _ =>    println!("Guilds table created.")
//     }
    
//     let token = dotenv::var("DISCORD_TOKEN")
//     .expect("Expected a token in the environment");

//     let http = Http::new_with_token(&token);
    
//     let (owners, _bot_id) = match http.get_current_application_info().await {
//         Ok(info) => {
//             let mut owners = HashSet::new();
//             owners.insert(info.owner.id);

//             (owners, info.id)
//         },
//         Err(e) => panic!("Could not get application info {:?}", e)
//     };


//     let framework = StandardFramework::new()
//             .configure(|c| c.owners(owners).prefix(dotenv::var("DISCORD_PREFIX").unwrap()))
//             .group(&GENERAL_GROUP);
//     let wch = cache_watch_channels().expect("Could not cache into WatchChannelHandler");
//     let wmem = cache_watched_members().expect("Could not cache into WatchMemeberHandler");

//     println!("\nLooking through WatchChannel cache.");
//     for (k, v) in &wch{
//         println!("{} - {}", k, v);
//     }
//     println!("\nLooking through WatchMember cache.");
//     for (k, v) in &wmem{
//         println!("{}", k);
//         for i in 0..v.len()-1{
//             println!("  ├─ {}", v[i]);
//         }
//         println!("  └─ {}", v[v.len()-1]);
//     }
//     let mut client =
//         Client::builder(&token)
//             .framework(framework)
//             .type_map_insert::<WatchChannelHandler>(wch)
//             .type_map_insert::<WatchMemberHandler>(wmem)
//             .event_handler(Handler)
//             .await
//             .expect("Could not create client");

//     if let Err(e) = client.start().await {
//         println!("Client error: {:?}", e);
//     }


// }

// use std::time::Duration;

// use db::sea_orm{ DatabaseConnection, Database, ConnectOptions, DbErr};
// use db::sea_orm;
// use serenity::Error;
// use tokio;
// use db::sea_orm::{Database, ConnectOptions, DbErr};
// #[tokio::main]
/*async*/ fn main() /* -> Result<(), DbErr> */ {
    // let mut opt = ConnectOptions::new("sqlite:./db.sqlite".to_owned());
    // opt
    //     .max_connections(100)
    //     .min_connections(5)
    //     .connect_timeout(Duration::from_secs(8))
    //     .idle_timeout(Duration::from_secs(8))
    //     .max_lifetime(Duration::from_secs(8))
    //     .sqlx_logging(true);

    // let db =  Database::connect(opt).await?;
    // db.as_mock_connection();
    // db::guild::Column::GuildId
    // Ok(())
}