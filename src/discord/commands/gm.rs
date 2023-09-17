use crate::{discord::{Context, Error}, rolls::roll_context::RollType};

#[poise::command(
    prefix_command,
    slash_command,
    subcommands(
        "fudge",
    ),
    subcommand_required
)]
pub async fn gm(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(
    prefix_command,
    slash_command,
    subcommands(
        "d4",
        "d6",
        "d20",
    ),
    subcommand_required
)]
pub async fn fudge(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(
    prefix_command,
    slash_command,
)]
pub async fn d4(ctx: Context<'_>, roll: i8) -> Result<(), Error> {
    {
        let mut state = ctx.data().tracker.lock().unwrap();
        state.context.d4 = RollType::FixedResult(roll);
    };
    println!("Roll fudged: d4={roll}");
    ctx.say("👍").await?;
    Ok(())
}

#[poise::command(
    prefix_command,
    slash_command,
)]
pub async fn d6(ctx: Context<'_>, roll: i8) -> Result<(), Error> {
    {
        let mut state = ctx.data().tracker.lock().unwrap();
        state.context.d6 = RollType::FixedResult(roll);
    };
    println!("Roll fudged: d6={roll}");
    ctx.say("👍").await?;
    Ok(())
}

#[poise::command(
    prefix_command,
    slash_command,
)]
pub async fn d20(ctx: Context<'_>, roll: i8) -> Result<(), Error> {
    {
        let mut state = ctx.data().tracker.lock().unwrap();
        state.context.d20 = RollType::FixedResult(roll);
    };
    println!("Roll fudged: d20={roll}");
    ctx.say("👍").await?;
    Ok(())
}
