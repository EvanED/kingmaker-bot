use futures::Stream;
use futures::StreamExt;

use crate::actions::c2_region::claim_hex::ClaimHexSkill;
use crate::{discord::{Context, Error}, spec::{Kingdom, skills::Skill}, turns::TurnState, state::{KingdomState, Commodity}, rolls::{roll_context::{RollContext, RollType}, roll_result::{RollResult, DC, DieRoll, NaturalRoll, TotalRoll, DegreeOfSuccess}}, actions::{b_commerce::{collect_taxes, improve_lifestyle, trade_commodities}, c1_leadership::{celebrate_holiday, create_a_masterpiece, prognostication, supernatural_solution, purchase_commodities, take_charge}, c2_region::{go_fishing, claim_hex, establish_farmland::{self, HexType}}, c3_civic::build_structure::{Structure, self}}, tracker::OverallState};
use std::str::FromStr;

#[poise::command(
    prefix_command,
    slash_command,
    subcommands(
        // Commerce phase
        "collect_taxes",
        "decline_to_collect_taxes",
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
        // Generic
        "comment",
        "next_turn",
    ),
    subcommand_required
)]
pub async fn act(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

fn post_action_updates(state: &mut OverallState) -> () {
    state.context.d4 = RollType::FairRoll;
    state.context.d6 = RollType::FairRoll;
    state.context.d20 = RollType::FairRoll;
}

pub async fn make_move<F>(ctx: Context<'_>, desc: &str, turn_func: F) -> Result<(), Error>
    where F: FnOnce(&Kingdom, &TurnState, &KingdomState, &RollContext) -> (RollResult, TurnState, KingdomState)
{
    let player = &ctx.author().name;
    let move_result = {
        let mut state = ctx.data().tracker.lock().unwrap();
        let move_result = state.make_move(
            player,
            desc.to_string(),
            turn_func,
        );
        post_action_updates(&mut state);
        move_result
    };
    let text = move_result.to_markdown(desc);
    println!("{text}");
    ctx.say(text).await?;
    Ok(())
}

/// A subcommand of `parent`
#[poise::command(
    prefix_command,
    slash_command,
    rename="collect-taxes",
)]
pub async fn collect_taxes(ctx: Context<'_>) -> Result<(), Error> {
    make_move(ctx, "Collect Taxes", &collect_taxes::collect_taxes).await
}

/// A subcommand of `parent`
#[poise::command(
    prefix_command,
    slash_command,
    rename="decline-to-collect-taxes",
)]
pub async fn decline_to_collect_taxes(ctx: Context<'_>) -> Result<(), Error> {
    make_move(ctx, "Decline To Collect Taxes", &collect_taxes::decline_to_collect).await
}

/// A subcommand of `parent`
#[poise::command(
    prefix_command,
    slash_command,
    rename="improve-lifestyle",
)]
pub async fn improve_lifestyle(ctx: Context<'_>) -> Result<(), Error> {
    make_move(ctx, "Improve Lifestyle", &improve_lifestyle::improve_lifestyle).await
}

/// A subcommand of `parent`
#[poise::command(
    prefix_command,
    slash_command,
    rename="trade-commodities",
)]
pub async fn trade_commodities(
    ctx: Context<'_>,
    commodity: Commodity,
    volume: i8,
) -> Result<(), Error> {
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
    rename="celebrate-holiday",
)]
pub async fn celebrate_holiday(ctx: Context<'_>) -> Result<(), Error> {
    make_move(ctx, "Celebrate Holiday", &celebrate_holiday::celebrate_holiday).await
}


/// A subcommand of `parent`
#[poise::command(
    prefix_command,
    slash_command,
    rename="create-a-masterpiece",
)]
pub async fn create_a_masterpiece(ctx: Context<'_>) -> Result<(), Error> {
    make_move(ctx, "Create a Masterpiece", &create_a_masterpiece::create_a_masterpiece).await
}


/// A subcommand of `parent`
#[poise::command(
    prefix_command,
    slash_command,
    rename="prognostication",
)]
pub async fn prognostication(ctx: Context<'_>) -> Result<(), Error> {
    make_move(ctx, "Prognostication", &prognostication::prognosticate).await
}


/// A subcommand of `parent`
#[poise::command(
    prefix_command,
    slash_command,
    rename="purchase-commodities",
)]
pub async fn purchase_commodities(
    ctx: Context<'_>,
    primary_want: Commodity,
    secondary_want: Commodity,
) -> Result<(), Error> {
    let closure = |kingdom: &_, turn: &_, state: &_, context: &_| {
        purchase_commodities::purchase_commodities(kingdom, turn, state, context, primary_want, secondary_want)
    };

    make_move(ctx, "Purchase Commodities", closure).await
}


/// A subcommand of `parent`
#[poise::command(
    prefix_command,
    slash_command,
    rename="supernatural-solution",
)]
pub async fn supernatural_solution(ctx: Context<'_>) -> Result<(), Error> {
    make_move(ctx, "Supernatural Solution", &supernatural_solution::supernatural_solution).await
}


/// A subcommand of `parent`
#[poise::command(
    prefix_command,
    slash_command,
    rename="take-charge",
)]
pub async fn take_charge(
    ctx: Context<'_>,
    skill: Skill,
) -> Result<(), Error> {
    let closure = |kingdom: &_, turn: &_, state: &_, context: &_| {
        take_charge::take_charge(kingdom, turn, state, context, skill)
    };

    let desc = format!("Take Charge ({skill:?})");
    make_move(ctx, &desc, closure).await
}

/////////////////////////////////////////////////



/// A subcommand of `parent`
#[poise::command(
    prefix_command,
    slash_command,
    rename="claim-hex",
)]
pub async fn claim_hex(
    ctx: Context<'_>,
    using_skill: ClaimHexSkill,
    x: i8,
    y: i8,
) -> Result<(), Error> {
    let closure = |kingdom: &_, turn: &_, state: &_, context: &_| {
        claim_hex::claim_hex(kingdom, turn, state, context, using_skill, x, y)
    };

    let desc = format!("Claim Hex {x}.{y}");

    make_move(ctx, &desc, closure).await
}


/// A subcommand of `parent`
#[poise::command(
    prefix_command,
    slash_command,
    rename="establish-farmland",
)]
pub async fn establish_farmland(
    ctx: Context<'_>,
    hex_type: HexType,
) -> Result<(), Error> {
    let closure = |kingdom: &_, turn: &_, state: &_, context: &_| {
        establish_farmland::establish_farmland(kingdom, turn, state, context, hex_type)
    };

    make_move(ctx, "Establish Farmland", closure).await
}


/// A subcommand of `parent`
#[poise::command(
    prefix_command,
    slash_command,
    rename="go-fishing",
)]
pub async fn go_fishing(ctx: Context<'_>) -> Result<(), Error> {
    make_move(ctx, "Go Fishing", &go_fishing::go_fishing).await
}

//////////////////////////////////////////////////

async fn autocomplete_structure<'a>(
    _ctx: Context<'_>,
    partial: &'a str,
) -> impl Stream<Item = String> + 'a {
    let matching = Structure::autocomplete_matching(partial);
    futures::stream::iter(matching)
        .filter(move |name| futures::future::ready(name.starts_with(partial)))
        .map(|name| name.to_string())
}

/// A subcommand of `parent`
#[poise::command(
    prefix_command,
    slash_command,
    rename="build-structure",
)]
pub async fn build_structure(
    ctx: Context<'_>,
    #[autocomplete = "autocomplete_structure"]
    structure: String,
) -> Result<(), Error> {
    let structure = Structure::from_str(&structure)?;
    let closure = |kingdom: &_, turn: &_, state: &_, context: &_| {
        build_structure::build_structure(kingdom, turn, state, context, structure)
    };

    make_move(ctx, "Build Structure", closure).await
}

#[poise::command(
    prefix_command,
    slash_command,
    rename="next-turn",
)]
pub async fn next_turn(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let closure = |_kingdom: &_, turn: &TurnState, state: &KingdomState, _context: &_| {
        let phony_rr = RollResult {
            dc: DC(0),
            die_roll: DieRoll {
                natural: NaturalRoll(0),
                total:   TotalRoll(0),
                description: "next turn".to_string(),
            },
            degree: DegreeOfSuccess::Success,
        };
        let nt = turn.next_turn();
        let nk = state.next_turn(&nt);
        (
            phony_rr, nt, nk,
        )
    };

    make_move(ctx, "Next Turn", closure).await
}

/// A subcommand of `parent`
#[poise::command(
    prefix_command,
    slash_command,
)]
pub async fn comment(
    ctx: Context<'_>,
    comment: String,
) -> Result<(), Error> {
    let closure = |_kingdom: &_, turn: &TurnState, state: &KingdomState, _context: &_| {
        let phony_rr = RollResult {
            dc: DC(0),
            die_roll: DieRoll {
                natural: NaturalRoll(0),
                total:   TotalRoll(0),
                description: "dummy".to_string(),
            },
            degree: DegreeOfSuccess::Success,
        };
        (
            phony_rr, turn.clone(), state.clone(),
        )
    };

    make_move(ctx, &comment, closure).await
}
