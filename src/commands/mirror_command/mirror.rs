
use std::{str::FromStr, string};

use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::{channel::Message, id::ChannelId},
    prelude::Context,
    utils::Colour,
};

use db::sea_orm::*;

use crate::commands::{get_channel_from_db, parse_channel_as_option};
use crate::utilities::permission_utilities::*;
use crate::{commands::parse_channel, Connection, MirrorChannelCache};

enum MirrorArgument {
    Channel(Option<i64>),
    List,
    Help,
    Remove,
}

impl std::str::FromStr for MirrorArgument {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "-r" | "-remove" => Ok(MirrorArgument::Remove),
            "-l" | "-list" => Ok(MirrorArgument::List),
            "-h" | "-help" | "-i" | "-info" => Ok(MirrorArgument::Help),
            x => Ok(MirrorArgument::Channel(parse_channel_as_option(x))),
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
#[aliases(
    mirror_channel,
    mirrorChannel,
    mirrorchannel,
    mirror_channel,
    mc,
    mirror
)]
async fn mirror(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if !is_message_author_admin(ctx, msg).await {
        msg.reply(ctx, "You must be a moderator to run this command.")
            .await?;
        return Ok(());
    }
    let arg_array = args.raw().collect::<Vec<&str>>();
    let message_channel_id = &i64::from(msg.channel_id);
    let message_guild_id = &i64::from(msg.guild_id.unwrap());
    let first_argument = arg_array[0];
    if arg_array.len() == 2 {
        let second_argument = arg_array[1];
        match [
            MirrorArgument::from(first_argument),
            MirrorArgument::from(second_argument),
        ] {
            // source INTO mirror
            [MirrorArgument::Channel(Some(x)), MirrorArgument::Channel(Some(y))] => match [
                get_channel_from_db(first_argument, &ctx, message_guild_id).await,
                get_channel_from_db(second_argument, &ctx, message_guild_id).await,
            ] {
                [Ok(Some(mut first)), Ok(Some(mut _second))] => {
                    let [source_channel_id, mirror_channel_id] = [x, y];
                    println!();
                    let mut data = ctx.data.write().await;
                    let con = data.get::<Connection>().unwrap();
                    println!("nice2");
                    first.mirror_to_channel_id = Set(Some(mirror_channel_id));
                    first.update(con).await?;
                    data.get_mut::<MirrorChannelCache>()
                        .unwrap()
                        .insert(source_channel_id, mirror_channel_id);
                }
                [Err(_e), Err(_y)] => {
                    println!("evil error")
                }
                [_, _] => {
                    println!(" ")
                }
            },
            // remove source ( no longer links to a mirror )
            [MirrorArgument::Remove, MirrorArgument::Channel(_id)] => {
                // TODO:
            }
            _ => {
                msg.reply(
                    &ctx,
                    "Invalid mirror arguments supplied. Please refer to mirror --help for usage",
                )
                .await?;
            }
        }
    } else if arg_array.len() == 1 {
        match MirrorArgument::from(first_argument) {
            MirrorArgument::Remove => {
                let mut data = ctx.data.write().await;
                let con = data.get::<Connection>().unwrap();
                if let Some(_) = data
                    .get::<MirrorChannelCache>()
                    .unwrap()
                    .get(message_channel_id)
                {
                    let mut x: db::channel::ActiveModel =
                        db::channel::Entity::find_by_id(message_channel_id.to_owned())
                            .one(con)
                            .await
                            .expect("oh no!")
                            .unwrap()
                            .into();
                    x.mirror_to_channel_id = Set(None);
                    x.update(con).await?;
                    data.get_mut::<MirrorChannelCache>()
                        .unwrap()
                        .remove(message_channel_id);
                }
            }
            MirrorArgument::Channel(_id) => {
                let mut data = ctx.data.write().await;
                let con = data.get::<Connection>().unwrap();
                match parse_channel(arg_array[0]) {
                    Ok(mirror_channel) => {
                        match db::channel::Entity::find_by_id(message_channel_id.to_owned())
                            .one(con)
                            .await
                        {
                            Ok(Some(model)) => {
                                let mut channel: db::channel::ActiveModel = model.into();
                                channel.mirror_to_channel_id = Set(Some(mirror_channel));
                                channel.update(con).await?;
                                data.get_mut::<MirrorChannelCache>()
                                    .unwrap()
                                    .insert(message_channel_id.to_owned(), mirror_channel);
                            }
                            Ok(None) => {
                                db::channel::ActiveModel {
                                    guild_id: Set(message_guild_id.to_owned()),
                                    channel_id: Set(message_channel_id.to_owned()),
                                    mirror_to_channel_id: Set(Some(mirror_channel)),
                                }
                                .insert(con)
                                .await
                                .expect("COULDNT DO IT LMAO WTF");
                                data.get_mut::<MirrorChannelCache>()
                                    .unwrap()
                                    .insert(message_channel_id.to_owned(), mirror_channel);
                            }
                            Err(_) => {
                                msg.reply(&ctx, "Valid Channel Not Supplied").await?;
                            }
                        }
                    }
                    Err(_) => {
                        msg.reply(ctx, "Please supply a channel to mirror into")
                            .await?;
                        return Ok(());
                    }
                }
            }
            MirrorArgument::List => {
                let data = ctx.data.write().await;
                let con = data.get::<Connection>().unwrap();
                let channels = db::channel::Entity::find()
                    .filter(
                        db::channel::Column::MirrorToChannelId
                            .eq(Some(message_channel_id.to_owned())),
                    )
                    .all(con)
                    .await
                    .expect("Mirror channel not found");

                let mut channel_response = string::String::from("");
                for channel in channels {
                    let n: ChannelId = ChannelId(channel.channel_id as u64);
                    let channel_name = &ctx.cache.guild_channel(n).await.unwrap().name;
                    channel_response += &format!("\n- #{}", &channel_name);
                }
                ChannelId(*message_channel_id as u64)
                    .send_message(&ctx, |m| {
                        m.embed(|e| {
                            e.color(Colour::LIGHTER_GREY)
                                .title("Source Channels")
                                .description(channel_response)
                        })
                    })
                    .await
                    .unwrap();
            }
            MirrorArgument::Help => {
                ChannelId(*message_channel_id as u64).send_message(&ctx, |m| {
                    m.embed(|e| {
                        e.color(Colour::FABLED_PINK)
                            .title("Mirror")
                            .description(format!("[required](optional) \"|\" denotes this OR that : [{}mirror|mirror_channel|mc] [-l|-list|-r|-remove|-h|-help|-i|-info|#mirroring_channel] \n (#source_channel) Mirror is the way chains handles logging messages, with the purpose of combining discussions from different channels 
                        for the purpose of having all server discussion in a single place for audit and moderation purposes", "-"))
                    })
                }).await.unwrap();
            }
        }
    }
    Ok(())
}
