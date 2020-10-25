use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
async fn wipe(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let amount = args.single::<String>()?;

    if amount.eq_ignore_ascii_case("all") {
        let messages = msg
            .channel_id
            .messages(&ctx, |retriever| retriever.before(msg.id))
            .await?;
        for message in messages {
            message.delete(&ctx).await?;
        }
        msg.delete(&ctx).await?;
    }

    else if amount.eq_ignore_ascii_case("after") {
        let after_id = args.single::<u64>()?;
        let messages = msg
            .channel_id
            .messages(&ctx, |retriever| retriever.after(MessageId(after_id)))
            .await?;
        for message in messages {
            message.delete(&ctx).await?;
        }
    }

    else if amount.eq_ignore_ascii_case("before") {
        let after_id = args.single::<u64>()?;
        let messages = msg
            .channel_id
            .messages(&ctx, |retriever| retriever.before(MessageId(after_id)))
            .await?;
        for message in messages {
            message.delete(&ctx).await?;
        }
        msg.delete(&ctx).await?;
    }

    else if amount.eq_ignore_ascii_case("start") {
        let after_id = args.single::<u64>()?;
        let msg_number = args.single::<u64>()?;
        let messages = msg
            .channel_id
            .messages(&ctx, |retriever| retriever.after(MessageId(after_id)).limit(msg_number))
            .await?;
        for message in messages {
            message.delete(&ctx).await?;
        }
        msg.delete(&ctx).await?;
    }

    else if amount.eq_ignore_ascii_case("end") {
        let after_id = args.single::<u64>()?;
        let msg_number = args.single::<u64>()?;
        let messages = msg
            .channel_id
            .messages(&ctx, |retriever| retriever.before(MessageId(after_id)).limit(msg_number))
            .await?;
        for message in messages {
            message.delete(&ctx).await?;
        }
        msg.delete(&ctx).await?;
    }

    Ok(())
}

//TODO: fix unwraps so that code doesn't panic upon recieving None
#[command]
async fn verify(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let guildid = msg.guild_id.unwrap();
    let guild = ctx.cache.guild(guildid).await.unwrap();
    let userid = args.single::<UserId>()?;
    let mut member = guildid.member(&ctx.http, userid).await.unwrap();
    let role = guild.role_by_name("member").unwrap();

    member.add_role(&ctx.http, role).await?;
    Ok(())
}

// note to fetch a list of all guild members
// it is necessary to have access to privileged intents
// #[command]
// async fn verifyall(ctx: &Context, msg: &Message) -> CommandResult {
//     let guildid = msg.guild_id.unwrap();
//     let members = guildid.members(&ctx.http, None, None).await.unwrap();
//     Ok(())
// }