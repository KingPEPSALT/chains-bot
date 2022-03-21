use crate::{Connection, EntityTrait, MirrorChannelCache};
use db::sea_orm::{ActiveModelTrait, ColumnTrait, IntoSimpleExpr, QueryFilter, Set, Value};
use serenity::client::Context;
use std::num::ParseIntError;

use db::sea_orm::sea_query::error::Error;

pub mod disclaimer;
pub mod mirror;
pub mod mod_role;
pub mod ping;
pub mod snapshot;
pub mod snapshot_channel;
pub mod watch;

pub fn parse_channel(channel_mention: &str) -> Result<i64, ParseIntError> {
    match channel_mention.parse::<i64>() {
        Ok(t) => Ok(t),
        Err(_) => channel_mention[2..channel_mention.len() - 1].parse::<i64>(),
    }
}

pub fn parse_channel_as_option(channel_mention: &str) -> Option<i64> {
    match channel_mention.parse::<i64>() {
        Ok(t) => Some(t),
        Err(_) => match channel_mention[2..channel_mention.len() - 1].parse::<i64>() {
            Ok(id) => Some(id),
            Err(_) => None,
        },
    }
}

// retrieve OR create a channel from an optional id. if id is none just return None for the model
async fn get_channel(
    channel_id: i64,
    ctx: &Context,
    guild_id: &i64,
) -> Result<Option<db::channel::ActiveModel>, Error> {
    let data = ctx.data.write().await;
    let con = data.get::<Connection>().expect("wow");
    return match db::channel::Entity::find_by_id(channel_id.to_owned())
        .one(con)
        .await
        .expect("nice")
    {
        Some(channel) => {
            let mut active_channel: db::channel::ActiveModel = channel.into();
            Ok(Some(active_channel))
        }
        None => {
            let mut active_channel: db::channel::ActiveModel = db::channel::ActiveModel {
                guild_id: Set(guild_id.to_owned()),
                channel_id: Set(channel_id.to_owned()),
                mirror_to_channel_id: Set(None),
            }
            .insert(con)
            .await
            .expect("Issue creating channel")
            .into();
            Ok(Some(active_channel))
        }
    };
}

pub async fn get_channel_from_db(
    channel_mention: &str,
    ctx: &Context,
    guild_id: &i64,
) -> Result<Option<db::channel::ActiveModel>, Error> {
    get_channel(
        parse_channel(channel_mention).expect("yippy"),
        ctx,
        guild_id,
    )
    .await
}
