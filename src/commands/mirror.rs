use core::fmt;
use std::str::FromStr;

use crate::{commands::parse_channel, Connection, MirrorChannelCache};
use db::sea_orm::{EntityTrait, Set, ActiveModelTrait};
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    prelude::Context,
};

use crate::utilities::permission_utilities::*;

enum MirrorArgument {
    Channel,
    Remove
}
impl std::str::FromStr for MirrorArgument {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "-r" => Ok(MirrorArgument::Remove),
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
                            db::channel::Entity::find_by_id(message_channel_id.to_owned())
                            .one(con).await
                            .expect("oh no!").unwrap().mirror_to_channel_id = None;
                            data.get_mut::<MirrorChannelCache>().unwrap().remove(message_channel_id);
                        }
                    },
                    _ => {
                    }
                }
            }
        }
    }
    else if arg_array.len() == 1{
        match MirrorArgument::from(first_argument) {
            MirrorArgument::Remove => {},
            MirrorArgument::Channel => {
                let mut data = ctx.data.write().await;
                let con = data.get::<Connection>().unwrap();
            }
        }
    }

    Ok(())
}