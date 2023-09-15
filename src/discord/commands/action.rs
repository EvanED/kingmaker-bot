use crate::{discord::{Context, Error}, actions::b_commerce::collect_taxes};

#[poise::command(
    prefix_command,
    slash_command,
    subcommands(
        // Commerce phase
        "collect_taxes",
        "improve_lifestyle",
        "trade_commodities",
        // Action phase, leadership actions
        "celebrate_holiday",
        "create_a_masterpiece",
        "prognostication",
        "purchase_commodities",
        "supernatural_solution",
        "take_charge",
        // Action phase, region actions
        "claim_hex",
        "establish_farmland",
        "go_fishing",
        // Action phase, civic actions
        "build_structure",
    ),
    subcommand_required
)]
pub async fn act(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// A subcommand of `parent`
#[poise::command(
    prefix_command,
    slash_command,
)]
pub async fn collect_taxes(ctx: Context<'_>) -> Result<(), Error> {
    let move_result = {
        let mut state = ctx.data().tracker.lock().unwrap();
        state.make_move(
            "Collecting Taxes".to_string(),
            &collect_taxes::collect_taxes,
        )
    };

    println!("..collect taxes: {:?}", move_result);

    ctx.say(format!("Collect Taxes: {:?}", move_result)).await?;
    Ok(())
}

/// A subcommand of `parent`
#[poise::command(
    prefix_command,
    slash_command,
)]
pub async fn improve_lifestyle(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Improve Lifestyle").await?;
    Ok(())
}

/// A subcommand of `parent`
#[poise::command(
    prefix_command,
    slash_command,
)]
pub async fn trade_commodities(
    ctx: Context<'_>,
    commodity: String,
    volume: i8,
) -> Result<(), Error> {
    let s = format!("Trade Commodities: {volume} {commodity}");
    ctx.say(s).await?;
    Ok(())
}

///////////////////////////////////


/// A subcommand of `parent`
#[poise::command(
    prefix_command,
    slash_command,
)]
pub async fn celebrate_holiday(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Celebrate Holiday").await?;
    Ok(())
}


/// A subcommand of `parent`
#[poise::command(
    prefix_command,
    slash_command,
)]
pub async fn create_a_masterpiece(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Create a Masterpiece").await?;
    Ok(())
}


/// A subcommand of `parent`
#[poise::command(
    prefix_command,
    slash_command,
)]
pub async fn prognostication(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Prognostication").await?;
    Ok(())
}


/// A subcommand of `parent`
#[poise::command(
    prefix_command,
    slash_command,
)]
pub async fn purchase_commodities(
    ctx: Context<'_>,
    primary_want: String,
    secondary_want: String,
) -> Result<(), Error> {
    let s = format!("Purchase Commodities: {primary_want} (secondary {secondary_want})");
    ctx.say(s).await?;
    Ok(())
}


/// A subcommand of `parent`
#[poise::command(
    prefix_command,
    slash_command,
)]
pub async fn supernatural_solution(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Supernatural Solution").await?;
    Ok(())
}


/// A subcommand of `parent`
#[poise::command(
    prefix_command,
    slash_command,
)]
pub async fn take_charge(
    ctx: Context<'_>,
    skill: String,
) -> Result<(), Error> {
    let s = format!("Take Charge: {skill}");
    ctx.say(s).await?;
    Ok(())
}

/////////////////////////////////////////////////


/// A subcommand of `parent`
#[poise::command(
    prefix_command,
    slash_command,
)]
pub async fn claim_hex(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Claim Hex").await?;
    Ok(())
}


/// A subcommand of `parent`
#[poise::command(
    prefix_command,
    slash_command,
)]
pub async fn establish_farmland(
    ctx: Context<'_>,
    hex_type: String,
) -> Result<(), Error> {
    let s = format!("Establish Farmland on {hex_type}");
    ctx.say(s).await?;
    Ok(())
}


/// A subcommand of `parent`
#[poise::command(
    prefix_command,
    slash_command,
)]
pub async fn go_fishing(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Go Fishing").await?;
    Ok(())
}

//////////////////////////////////////////////////

/// A subcommand of `parent`
#[poise::command(
    prefix_command,
    slash_command,
)]
pub async fn build_structure(
    ctx: Context<'_>,
    structure: String,
) -> Result<(), Error> {
    let s = format!("Build Structure: {structure}");
    ctx.say(s).await?;
    Ok(())
}
