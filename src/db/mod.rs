// pub mod model;

// use rusqlite::{params, Connection, Result, Error as DBError, params_from_iter};
// use model::Guild;
// use std::{sync::Arc, iter::once, collections::HashMap};
// use fallible_iterator::FallibleIterator;

// const PATH : &str = "../db.sqlite";
// const DEFAULT_GUILD : Guild = Guild{
//     guild_id: 0,
//     snapshot_channel: None,
//     mod_role: None,
//     disclaimer_compliant: false,
// };

// type GuildResult = Result<Guild, DBError>;
// type ExecutionResult = Result<(), DBError>;

// thread_local!(static CONNECTION: Arc<Connection> = Arc::new(Connection::open(&PATH).unwrap()));

// let mut opt = ConnectOptions::new();


// pub fn create_guilds_table() -> ExecutionResult{
    
//     let connection = Connection::open(&PATH)?;
//     connection.execute(
//         "CREATE TABLE Guilds (
//             guildId                 INTEGER PRIMARY KEY     NOT NULL,
//             snapChannelId           INTEGER,                
//             modRoleId               INTEGER,
//             disclaimerCompliancy    INTEGER                 DEFAULT 0 NOT NULL,
//             watchChannelId          INTEGER
//         )",
//         []
//     ).and_then(|_| Ok(()))
    
// }
// pub fn create_watched_members_table() -> ExecutionResult{
//     let connection = Connection::open(&PATH)?;
//     connection.execute(
//         "CREATE TABLE WatchedMembers (
//             guildId          INTEGER         NOT NULL,
//             userId           INTEGER         NOT NULL,
//             PRIMARY KEY (userId, guildId)
//         )",
//         // this should be a part of the SQL but doesnt work currently.
//         // FOREIGN KEY (guildId) REFERENCES Guilds (guildId) 
//         []
//     ).and_then(|_| Ok(()))
// }

// /* Helper function 
// pub fn add_column() -> ExecutionResult{
//     let connection = Connection::open(&PATH)?;
//     connection.execute("ALTER TABLE Guild ADD COLUMN disclaimer_compliant INTEGER", []).and_then(|_| Ok(()))
// }
// */

// pub fn delete_guild(guild_id: &u64) -> ExecutionResult{
//     CONNECTION.with(|con|{
//         con.execute("DELETE FROM Guilds WHERE guildId = ?", params![guild_id])?;
//         con.execute("DELETE FROM WatchedMembers WHERE guildId = ?", params![guild_id]).and_then(|_| Ok(()))
//     })
// }

// pub fn cache_watched_members() -> Result<HashMap<u64, Vec<u64>>>{
//     CONNECTION.with(|con|{
//         let mut stmt = con.prepare("SELECT * FROM WatchedMembers")?;
//         let mut rows = stmt.query([])?;
//         let mut data_cache: HashMap<u64, Vec<u64>> = HashMap::new();
//         while let Some(row) = rows.next().unwrap(){
//             match data_cache.get_mut(&row.get(0)?){
//                 Some(t) => {
//                     t.push(row.get(1)?);
//                 },
//                 None => {
//                     data_cache.insert(row.get(0)?, vec![row.get(1)?]);
//                 }
//             };
//         }
//         Ok(data_cache)
//     })
// }

// pub fn cache_watch_channels() -> Result<HashMap<u64, u64>>{
//     CONNECTION.with(|con|{
//         let mut stmt = con.prepare("SELECT guildId, watchChannelId From Guilds")?;
//         let mut rows = stmt.query([])?;
//         let mut data_cache: HashMap<u64, u64> = HashMap::new();
//         while let Some(row) = rows.next().unwrap(){
//             data_cache.insert(row.get(0)?, row.get::<_, u64>(1).or::<u64>(Ok(0)).unwrap());
//         }
//         Ok(data_cache)
//     })
// }
// pub fn add_watch_channel(guild_id: &u64, channel_id: &u64, cache: &mut HashMap<u64, u64>) -> ExecutionResult{

//     CONNECTION.with(|con|{
//         con.execute(
//             "UPDATE Guilds SET watchChannelId = ?1 WHERE guildId = ?2", params![channel_id, guild_id]
//         )?;
//         cache.insert(*guild_id, *channel_id);
//         Ok(())
//     })
// }
// pub fn add_watched_member(guild_id: &u64, user_id: &u64, cache: &mut HashMap<u64, Vec<u64>>) -> ExecutionResult{
//     CONNECTION.with(|con|{
//         con.execute(
//             "INSERT INTO WatchedMembers VALUES (?1, ?2)", params![guild_id, user_id]
//         )?;
//         match cache.get_mut(guild_id){
//             Some(t) => {
//                 t.push(*user_id);
//             },
//             None => {
//                 cache.insert(*guild_id, vec![*user_id]);
//             }
//         };
//         Ok(())
//     })
// }
// pub fn remove_watched_member(guild_id: &u64, user_id: &u64, cache: &mut HashMap<u64, Vec<u64>>) -> ExecutionResult{
//     CONNECTION.with(|con|{
//         con.execute(
//             "DELETE FROM WatchedMembers WHERE guildId = ?1 AND userId = ?2", params![guild_id, user_id]
//         )?;
//         match cache.get_mut(guild_id){
//             Some(t) => {
//                 t.remove(t.iter().position(|id| id == user_id).unwrap()); // should maybe refactor to use a hashable set or something
//             },
//             _ => ()
//         };
//         Ok(())
//     })
// }
// pub fn select(why_params: &[(&str, &str)]) -> Result<Vec<Guild>, DBError>{
//     // 455 character one liner...
//     CONNECTION.with::<_, Result<Vec<Guild>, DBError>>(|con|{
//         con.prepare(format!("SELECT * FROM Guilds WHERE {}", &"?=?,".repeat(why_params.len())[0..(why_params.len()*4-1)]).as_str())?.query(
//             params_from_iter(
//                 why_params.iter().flat_map(
//                     |tuple| once(tuple.0).chain(once(tuple.1))
//                 )
//             )
//         )?.map(|row| 
//             Ok(
//                 Guild{
//                     guild_id: row.get(0)?,
//                     snapshot_channel: row.get(1)?,
//                     mod_role: row.get(2)?,
//                     disclaimer_compliant: row.get(3)?
//                 }
//             )
//         ).collect::<Vec<Guild>>()
//     })

// }

// pub fn get_guild(guild_id: &u64) -> GuildResult{
//     CONNECTION.with(|con|{
//         con.query_row("SELECT * FROM Guilds WHERE guildId = ?", &[guild_id], |row| {
//             Ok(Guild{
//                 guild_id: row.get(0)?,
//                 snapshot_channel: row.get(1)?,
//                 mod_role: row.get(2)?,
//                 disclaimer_compliant: row.get(3)?
//             })
//         })
//     })
     
// }
// pub fn add_guild(guild_id: &u64) -> GuildResult{
//     CONNECTION.with(|con|{
//         match con.execute("INSERT INTO Guilds VALUES (?, NULL, NULL, 0, NULL)", params![guild_id]) {
//             Ok(_) => Ok(Guild{
//                 guild_id: *guild_id,
//                 ..DEFAULT_GUILD
//             }),
//             Err(e) => return Err(e)
//         }
//     })
// }

// pub fn update_snapshot_channel(guild_id: &u64, channel_id: &u64) -> ExecutionResult{
//     CONNECTION.with(|con|{
//         con.execute("UPDATE Guilds SET snapChannelId = ?1 WHERE guildId = ?2", params![channel_id, guild_id]).and_then(|_| Ok(()))
//     })
// }

// pub fn update_mod_role(guild_id: &u64, role_id: &u64) -> ExecutionResult {
//     CONNECTION.with(|con|{
//         con.execute("UPDATE Guilds SET modRoleId = ?1 WHERE guildId = ?2", params![role_id, guild_id]).and_then(|_| Ok(()))
//     })
// }

// pub fn update_compliancy(guild_id: &u64, compliancy: bool) -> ExecutionResult {
//     CONNECTION.with(|con|{
//         con.execute("UPDATE Guilds SET disclaimerCompliancy = ?1 WHERE guildId = ?2", params![&(compliancy as i32), guild_id]).and_then(|_| Ok(()))
//     })
// }

// pub fn clear_compliancies()  -> ExecutionResult{
//     CONNECTION.with(|con|{
//         con.execute("UPDATE Guilds SET disclaimerCompliancy = FALSE WHERE TRUE", []).and_then(|_| Ok(()))
//     })
// }