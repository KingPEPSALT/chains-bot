pub mod model;

use rusqlite::{params, Connection, Result, Error as DBError};
use model::Guild;
use std::sync::Arc;

const PATH : &str = "../db.sqlite";
const DEFAULT_GUILD : Guild = Guild{
    guild_id: 0,
    snapshot_channel: None,
    mod_role: None,
    disclaimer_compliant: false,
};
/* Helper function
pub fn create_database() -> Result<Connection, DBError>{
    
    let connection = Connection::open(&PATH)?;
    match connection.execute(
        "CREATE TABLE guild (
            id              STRING PRIMARY KEY,
            snap_channel    STRING,
            mod_role        STRING
        )",
        [],
    ){
        Ok(_) => Ok(connection),
        Err(e) => Err(e)
    }
    
}
*/

thread_local!(static CONNECTION: Arc<Connection> = Arc::new(Connection::open(&PATH).unwrap()));

pub fn get_guild(_guild_id: &u64) -> Result<Guild, DBError>{
    CONNECTION.with(|con|{
        con.query_row("SELECT * FROM guild WHERE id = ?", &[_guild_id], |row| {
            Ok(Guild{
                guild_id: row.get(0)?,
                snapshot_channel: row.get(1)?,
                mod_role: row.get(2)?,
                disclaimer_compliant: row.get(3)?
            })
        })
    })
     
}
pub fn add_guild(_guild_id: &u64) -> Result<Guild, DBError>{
    CONNECTION.with(|con|{
        match con.execute("INSERT INTO guild VALUES (?, NULL, NULL)", params![_guild_id]) {
            Ok(_) => Ok(Guild{
                guild_id: *_guild_id,
                ..DEFAULT_GUILD
            }),
            Err(e) => return Err(e)
        }
    })
}

pub fn update_snapshot_channel(_guild_id: &u64, _channel_id: &u64) -> Result<(), DBError> {
    CONNECTION.with(|con|{
        match con.execute("UPDATE guild SET snap_channel = ?1 WHERE id = ?2", params![_channel_id, _guild_id]){
            Ok(_) => Ok(()),
            Err(e) => Err(e)
        }
    })
}

pub fn update_mod_role(_guild_id: &u64, _role_id: &u64) -> Result<(), DBError> {
    CONNECTION.with(|con|{
        match con.execute("UPDATE guild SET mod_role = ?1 WHERE id = ?2", params![_role_id, _guild_id]){
            Ok(_) => Ok(()),
            Err(e) => Err(e)
        }
    })
}

pub fn update_compliancy(_guild_id: &u64, compliancy: bool) -> Result<(), DBError> {
    CONNECTION.with(|con|{
        match con.execute("UPDATE guild SET disclaimer_compliant = ?1 WHERE id = ?2", params![&(compliancy as i32), _guild_id]){
            Ok(_) => Ok(()),
            Err(e) => Err(e)
        }
    })
}
/* Helper function 
pub fn add_column() -> Result<(), DBError>{
    let connection = Connection::open(&PATH)?;
    match connection.execute("ALTER TABLE guild ADD COLUMN disclaimer_compliant INTEGER", []) {
        Ok(_) => Ok(()),
        Err(e) => Err(e)
    }
    
}
*/
pub fn clear_compliancies()  -> Result<(), DBError>{
    CONNECTION.with(|con|{
        match con.execute("UPDATE guild SET disclaimer_compliant = FALSE WHERE TRUE", []){
            Ok(_) => Ok(()),
            Err(e) => Err(e)
        }
    })
}