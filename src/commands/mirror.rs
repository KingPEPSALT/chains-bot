use core::fmt;
use std::{str::FromStr, string};

use crate::{commands::parse_channel, Connection, MirrorChannelCache};
use db::sea_orm::{EntityTrait, Set, ActiveModelTrait, ColumnTrait, QueryFilter, Value, IntoSimpleExpr};
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::{channel::Message, id::ChannelId},
    prelude::Context,
};

use crate::utilities::permission_utilities::*;

enum MirrorArgument {
    Channel,
    List,
    Remove
}
impl std::str::FromStr for MirrorArgument {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "-r" => Ok(MirrorArgument::Remove),
            "-l" => Ok(MirrorArgument::List),
            _ => Ok(MirrorArgument::Channel)
        }
    }
}
impl MirrorArgument {
    fn from(s: &str) -> Self {
        MirrorArgument::from_str(s).unwrap()
    }
}

#[command]
#[min_args(1)]
#[max_args(2)]
#[aliases(mirror_channel, mirrorChannel, mirrorchannel, mirror_channel, mc, mirror)]
async fn mirror(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    if !is_message_author_admin(ctx, msg).await{
        msg.reply(ctx, "You must be a moderator to run this command.").await?;
        return Ok(())
    }

    let arg_array = args.raw().collect::<Vec<&str>>();
    let message_channel_id = &i64::from(msg.channel_id);
    let message_guild_id = &i64::from(msg.guild_id.unwrap());
    let first_argument = arg_array[0];
    if arg_array.len() == 2 {
        let second_argument = arg_array[1];
        match [MirrorArgument::from(first_argument), MirrorArgument::from(second_argument)] {
            [_, MirrorArgument::Remove] => {
                msg.reply(&ctx, "Mirror must be either channel_id, channel_id, or -r channel_id to remove the mirror channel").await?;
                return Ok(());
            },
            _ => {
                let mut data = ctx.data.write().await;
                let con = data.get::<Connection>().unwrap();
                match MirrorArgument::from(first_argument) {
                    MirrorArgument::Remove => {
                        if let Some(_) = data.get::<MirrorChannelCache>().unwrap().get(message_channel_id) {
                            let mut x: db::channel::ActiveModel = db::channel::Entity::find_by_id(message_channel_id.to_owned())
                            .one(con).await
                            .expect("oh no!").unwrap().into();
                            msg.reply(&ctx, "poopy").await?;
                            x.mirror_to_channel_id = Set(None);
                            x.update(con).await?;
                            data.get_mut::<MirrorChannelCache>().unwrap().remove(message_channel_id);
                        }
                    },
                    _ => {
                        //TODO:
                    }
                }
            }
        }
    }
    else if arg_array.len() == 1{
        match MirrorArgument::from(first_argument) {
            MirrorArgument::Remove => {
                //TODO:
            },
            MirrorArgument::Channel => {
                let mut data = ctx.data.write().await;
                let con = data.get::<Connection>().unwrap();
                match parse_channel(arg_array[0]) {
                    Ok(mirror_channel) => {
                        match db::channel::Entity::find_by_id(message_channel_id.to_owned()).one(con).await {
                            Ok(Some(model)) => {
                                let mut channel: db::channel::ActiveModel = model.into();
                                channel.mirror_to_channel_id = Set(Some(mirror_channel));
                                channel.update(con).await?;
                                data.get_mut::<MirrorChannelCache>().unwrap().insert(message_channel_id.to_owned(), mirror_channel);
                            },
                            Ok(None) => {
                                db::channel::ActiveModel {
                                    guild_id: Set(message_guild_id.to_owned()),
                                    channel_id: Set(message_channel_id.to_owned()),
                                    mirror_to_channel_id: Set(Some(mirror_channel))
                                }.insert(con).await
                                .expect("COULDNT DO IT LMAO WTF HAHAHAHHA");
                                data.get_mut::<MirrorChannelCache>().unwrap().insert(message_channel_id.to_owned(), mirror_channel);
                            },
                            Err(_) => {
                                // TODO:
                            }
                        }
                    },
                    Err(_) => {
                        msg.reply(ctx, "Please supply a channel to mirror into").await?;
                        return Ok(());  
                    }
                }
            },
            MirrorArgument::List => {
                let data = ctx.data.write().await;
                msg.reply(&ctx, "ok?").await?;
                let con = data.get::<Connection>().unwrap();
                let channels = db::channel::Entity::find()
                    .filter(db::channel::Column::MirrorToChannelId.eq(Some(message_channel_id.to_owned())))
                    .all(con).await.expect("error here 1");
                msg.reply(&ctx, "ok? 2").await?;
                
                let mut channel_response = string::String::new();
                for channel in channels {
                    let n: ChannelId = ChannelId(channel.channel_id as u64);
                    let channel_name = &ctx.cache.guild_channel(n).await.unwrap().name;
                    channel_response += &format!("\n {}", &channel_name);
                }
                msg.reply(&ctx, channel_response).await.expect("error here 2");
            }
        }
    }

    Ok(())
}