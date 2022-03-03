pub mod model;

use rusqlite::{params, Connection, Result, Error as DBError};
use model::Guild;

const PATH : &str = "../db.sqlite";
const DEFAULT_GUILD : Guild = Guild{
    guild_id: 0,
    logging_channel_id: None,
    moderation_role_id: None
};
pub fn create_database() -> Result<Connection, DBError>{

    let connection = Connection::open(&PATH)?;
    match connection.execute(
        "CREATE TABLE guild (
            id          STRING PRIMARY KEY,
            log_chan    STRING,
            mod_role    STRING
        )",
        [],
    ){
        Ok(_) => Ok(connection),
        Err(e) => Err(e)
    }

}
pub fn get_guild(_guild_id: &u64) -> Result<Guild, DBError>{
    let connection = Connection::open(&PATH)?;
    connection.query_row("SELECT * FROM guild WHERE id = ?", &[_guild_id], |row| {Ok(Guild{
        guild_id: row.get(0)?,
        logging_channel_id: row.get(1)?,
        moderation_role_id: row.get(2)?
    })})
     
}
pub fn add_guild(_guild_id: &u64) -> Result<Guild, DBError>{
    let connection = Connection::open(&PATH)?;
    match connection.execute("INSERT INTO guild VALUES (?, NULL, NULL)", params![_guild_id]) {
        Ok(t) => Ok(Guild{
            guild_id: *_guild_id,
            ..DEFAULT_GUILD
        }),
        Err(e) => return Err(e)
    }
}
pub fn update_logging_channel(_guild_id: &u64, _channel_id: &u64) -> Result<(), DBError> {
    let connection = Connection::open(&PATH)?;
    match connection.execute("UPDATE guild SET log_chan = ?1 WHERE id = ?2", params![_channel_id, _guild_id]){
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
