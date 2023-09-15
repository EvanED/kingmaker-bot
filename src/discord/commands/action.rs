use crate::{discord::{Context, Error}, spec::{Kingdom, skills::Skill}, turns::TurnState, state::{KingdomState, Commodity}, rolls::roll_context::RollContext, actions::{b_commerce::{collect_taxes, improve_lifestyle, trade_commodities}, c1_leadership::{celebrate_holiday, create_a_masterpiece, prognostication, supernatural_solution, purchase_commodities, take_charge}, c2_region::{go_fishing, claim_hex, establish_farmland::{self, HexType}}, c3_civic::build_structure::{Structure, self}}};
use std::str::FromStr;

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

pub async fn make_move<F>(ctx: Context<'_>, desc: &str, turn_func: F) -> Result<(), Error>
    where F: FnOnce(&Kingdom, &TurnState, &KingdomState, &RollContext) -> (TurnState, KingdomState)
{
    let move_result = {
        let mut state = ctx.data().tracker.lock().unwrap();
        state.make_move(
            desc.to_string(),
            turn_func,
        )
    };
    println!("..{desc}: {:?}", move_result);
    ctx.say(format!("{desc}: {:?}", move_result)).await?;
    Ok(())
}

/// A subcommand of `parent`
#[poise::command(
    prefix_command,
    slash_command,
)]
pub async fn collect_taxes(ctx: Context<'_>) -> Result<(), Error> {
    let closure = |kingdom: &_, turn: &_, state: &_, context: &_| {
        let triple = collect_taxes::collect_taxes(kingdom, turn, state, context);
        let (_, turn_state, kingdom_state) = triple;
        (turn_state, kingdom_state)
    };
    make_move(ctx, "Collect Taxes", closure).await
}

/// A subcommand of `parent`
#[poise::command(
    prefix_command,
    slash_command,
)]
pub async fn improve_lifestyle(ctx: Context<'_>) -> Result<(), Error> {
    make_move(ctx, "Improve Lifestyle", &improve_lifestyle::improve_lifestyle).await
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
    let commodity = Commodity::from_str(&commodity)?;
    let closure = |kingdom: &_, turn: &_, state: &_, context: &_| {
        trade_commodities::trade_commodities(kingdom, turn, state, context, commodity, volume)
    };

    make_move(ctx, "Trade Commodities", closure).await
}

///////////////////////////////////


/// A subcommand of `parent`
#[poise::command(
    prefix_command,
    slash_command,
)]
pub async fn celebrate_holiday(ctx: Context<'_>) -> Result<(), Error> {
    make_move(ctx, "Celebrate Holiday", &celebrate_holiday::celebrate_holiday).await
}


/// A subcommand of `parent`
#[poise::command(
    prefix_command,
    slash_command,
)]
pub async fn create_a_masterpiece(ctx: Context<'_>) -> Result<(), Error> {
    make_move(ctx, "Create a Masterpiece", &create_a_masterpiece::create_a_masterpiece).await
}


/// A subcommand of `parent`
#[poise::command(
    prefix_command,
    slash_command,
)]
pub async fn prognostication(ctx: Context<'_>) -> Result<(), Error> {
    make_move(ctx, "Prognostication", &prognostication::prognosticate).await
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
    let primary_want = Commodity::from_str(&primary_want)?;
    let secondary_want = Commodity::from_str(&secondary_want)?;
    let closure = |kingdom: &_, turn: &_, state: &_, context: &_| {
        purchase_commodities::purchase_commodities(kingdom, turn, state, context, primary_want, secondary_want)
    };

    make_move(ctx, "Purchase Commodities", closure).await
}


/// A subcommand of `parent`
#[poise::command(
    prefix_command,
    slash_command,
)]
pub async fn supernatural_solution(ctx: Context<'_>) -> Result<(), Error> {
    make_move(ctx, "Supernatural Solution", &supernatural_solution::supernatural_solution).await
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
    let skill = Skill::from_str(&skill)?;
    let closure = |kingdom: &_, turn: &_, state: &_, context: &_| {
        take_charge::take_charge(kingdom, turn, state, context, skill)
    };

    make_move(ctx, "Take Charge", closure).await
}

/////////////////////////////////////////////////


/// A subcommand of `parent`
#[poise::command(
    prefix_command,
    slash_command,
)]
pub async fn claim_hex(ctx: Context<'_>, using_skill: String) -> Result<(), Error> {
    let skill = Skill::from_str(&using_skill)?;
    let closure = |kingdom: &_, turn: &_, state: &_, context: &_| {
        claim_hex::claim_hex(kingdom, turn, state, context, skill)
    };

    make_move(ctx, "Claim Hex", closure).await
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
    let hex_type = HexType::from_str(&hex_type)?;
    let closure = |kingdom: &_, turn: &_, state: &_, context: &_| {
        establish_farmland::establish_farmland(kingdom, turn, state, context, hex_type)
    };

    make_move(ctx, "Claim Hex", closure).await
}


/// A subcommand of `parent`
#[poise::command(
    prefix_command,
    slash_command,
)]
pub async fn go_fishing(ctx: Context<'_>) -> Result<(), Error> {
    make_move(ctx, "Go Fishing", &go_fishing::go_fishing).await
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
    let structure = Structure::from_str(&structure)?;
    let closure = |kingdom: &_, turn: &_, state: &_, context: &_| {
        build_structure::build_structure(kingdom, turn, state, context, structure)
    };

    make_move(ctx, "Claim Hex", closure).await
}
