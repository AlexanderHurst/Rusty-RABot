use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
async fn newgroup(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    println!("Creating private Channel");

    let guild = msg.guild(&ctx).await.expect("Failed to obtain Guild ID");

    let base_name = args.single::<String>()?;

    let category = guild
        .create_channel(&ctx, |c| {
            c.name(format!("{} {}", base_name, "Category"))
                .kind(ChannelType::Category)
        })
        .await?;

    let voice_chat = guild
        .create_channel(&ctx, |c| {
            c.name(format!("{} {}", base_name, "Voice"))
                .category(&category)
                .kind(ChannelType::Voice)
        })
        .await?;

    let text_chat = guild
        .create_channel(&ctx, |c| {
            c.name(format!("{} {}", base_name, "Text"))
                .category(&category)
                .kind(ChannelType::Text)
        })
        .await?;

    let bot_chat = guild
        .create_channel(&ctx, |c| {
            c.name(format!("{} {}", base_name, "Bot"))
                .category(&category)
                .kind(ChannelType::Text)
        })
        .await?;
    Ok(())
}

#[command]
async fn delgroup(_: &Context, _: &Message) -> CommandResult {
    unimplemented!();
    // Ok(())
}
