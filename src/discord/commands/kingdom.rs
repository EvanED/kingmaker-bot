use crate::actions::c3_civic::build_structure::Structure;
use crate::discord::Context;
use crate::discord::Error;
use crate::rolls::bonus::AppliesTo;
use crate::rolls::bonus::AppliesUntil;
use crate::rolls::bonus::Bonus;
use crate::rolls::bonus::BonusType;
use crate::spec::Kingdom;
use crate::spec::attributes::Attribute;
use crate::spec::skills::Skill;
use crate::spec::skills::TrainingLevel;
use crate::state::Commodity;
use crate::state::KingdomState;
use crate::turns::RandomEventSelectionMethod;
use crate::turns::TurnState;
use enum_map::enum_map;
use std::convert::TryFrom;

pub fn create_aryc() -> Kingdom {
    use Attribute::*;
    use Skill::*;
    use TrainingLevel::*;
    let aryc = Kingdom {
        level: 1,
        attributes: enum_map! {
            Culture   => 0,
            Economy   => 4,
            Loyalty   => 2,
            Stability => 2,
        },
        invested: enum_map! {
            Culture   => true,
            Economy   => true,
            Loyalty   => true,
            Stability => true,
        },
        skills: enum_map! {
            Agriculture => Untrained,
            Arts        => Trained,
            Boating     => Trained,
            Defense     => Trained,
            Engineering => Trained,
            Exploration => Untrained,
            Folklore    => Untrained,
            Industry    => Trained,
            Intrigue    => Untrained,
            Magic       => Trained,
            Politics    => Trained,
            Scholarship => Untrained,
            Statecraft  => Untrained,
            Trade       => Trained,
            Warfare     => Trained,
            Wilderness  => Untrained,
        },
    };
    aryc
}

fn _create_kingdom_state() -> KingdomState {
    KingdomState {
        unrest: 2,
        resource_points: 7,
        fame_points: 1,
        commodity_stores: enum_map! {
            Commodity::Food     => 1,
            Commodity::Lumber   => 2,
            Commodity::Luxuries => 0,
            Commodity::Ore      => 1,
            Commodity::Stone    => 3,
        },
    }
}

fn _create_turn_state() -> TurnState {
    TurnState {
        bonuses: vec![
            Bonus {
                type_         : BonusType::Circumstance,
                applies_to    : AppliesTo::RandomEventResolutions,
                applies_until : AppliesUntil::StartOfTheNextTurn,
                modifier      : 2,
                reason        : "whatever".to_string(),
            },
            Bonus {
                type_         : BonusType::Status,
                applies_to    : AppliesTo::Skill(Skill::Arts),
                applies_until : AppliesUntil::StartOfTheNextTurn,
                modifier      : -1,
                reason        : "something".to_string(),
            },
        ],
        requirements: vec![
            "do something".to_string(),
        ],

        create_a_masterpiece_attempted: false,
        supernatural_solution_available: true,
        random_event_selection_method: Some(RandomEventSelectionMethod::AdvantageGM),
        dc6_crop_failure_potential_for_x_turns: 0,

        collected_taxes: true,
        traded_commodities: false,
        bonus_rp: 1,
        additional_fame_points_scheduled: 0,
        supernatural_solution_blocked_for_x_turns: None,
        can_build_this_structure_for_no_resource_cost: Some(Structure::Cemetery),
    }
}

/// Roll a skill
///
/// Some skill to roll
///
/// Blah blah
#[poise::command(slash_command, prefix_command)]
async fn show(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let markdown = {
        let state = ctx.data().tracker.lock().unwrap();

        let kingdom = &state.kingdom;
        let kst = &state.turns.last().unwrap().kingdom_state;
        let tst = &state.turns.last().unwrap().turn_state;

        let markdown = format!(
            "{}{}{}",
            kingdom.to_markdown(),
            kst.to_markdown(),
            tst.to_markdown(),
        );
        markdown
    };

    println!("{markdown}");

    ctx.say(
        markdown
    ).await?;

    Ok(())
}

#[poise::command(slash_command, prefix_command)]
async fn history_dbg(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let state_str = {
        let state = ctx.data().tracker.lock().unwrap();
        format!("{:?}", state)
    };

    println!("{}", state_str);
    ctx.reply(state_str).await?;

    Ok(())
}

#[poise::command(
    prefix_command,
    slash_command,
    subcommands(
        "show",
        "history_dbg",
        "set",
        "discharge",
    ),
    subcommand_required
)]
pub async fn kingdom(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(
    prefix_command,
    slash_command,
    subcommands(
        "unrest",
        "rp",
        "fame",
        "food",
        "lumber",
        "luxuries",
        "ore",
        "stone",
    ),
    subcommand_required
)]
pub async fn set(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

fn set_unrest(turn_state: &TurnState, kingdom_state: &KingdomState, changer: Box<dyn FnOnce(i8) -> i8>) -> (TurnState, KingdomState) {
    let next_turn_state = turn_state.clone();
    let mut next_kingdom_state = kingdom_state.clone();

    next_kingdom_state.unrest = changer(next_kingdom_state.unrest);

    (next_turn_state, next_kingdom_state)
}

fn set_rp(turn_state: &TurnState, kingdom_state: &KingdomState, changer: Box<dyn FnOnce(i8) -> i8>) -> (TurnState, KingdomState) {
    let next_turn_state = turn_state.clone();
    let mut next_kingdom_state = kingdom_state.clone();

    next_kingdom_state.resource_points = changer(next_kingdom_state.resource_points);

    (next_turn_state, next_kingdom_state)
}

fn set_fame(turn_state: &TurnState, kingdom_state: &KingdomState, changer: Box<dyn FnOnce(i8) -> i8>) -> (TurnState, KingdomState) {
    let next_turn_state = turn_state.clone();
    let mut next_kingdom_state = kingdom_state.clone();

    next_kingdom_state.fame_points = changer(next_kingdom_state.fame_points);

    (next_turn_state, next_kingdom_state)
}

fn set_food(turn_state: &TurnState, kingdom_state: &KingdomState, changer: Box<dyn FnOnce(i8) -> i8>) -> (TurnState, KingdomState) {
    let next_turn_state = turn_state.clone();
    let mut next_kingdom_state = kingdom_state.clone();

    const COMMODITY: Commodity = Commodity::Food;
    next_kingdom_state.commodity_stores[COMMODITY] = changer(next_kingdom_state.commodity_stores[COMMODITY]);

    (next_turn_state, next_kingdom_state)
}

fn set_lumber(turn_state: &TurnState, kingdom_state: &KingdomState, changer: Box<dyn FnOnce(i8) -> i8>) -> (TurnState, KingdomState) {
    let next_turn_state = turn_state.clone();
    let mut next_kingdom_state = kingdom_state.clone();

    const COMMODITY: Commodity = Commodity::Lumber;
    next_kingdom_state.commodity_stores[COMMODITY] = changer(next_kingdom_state.commodity_stores[COMMODITY]);

    (next_turn_state, next_kingdom_state)
}

fn set_luxuries(turn_state: &TurnState, kingdom_state: &KingdomState, changer: Box<dyn FnOnce(i8) -> i8>) -> (TurnState, KingdomState) {
    let next_turn_state = turn_state.clone();
    let mut next_kingdom_state = kingdom_state.clone();

    const COMMODITY: Commodity = Commodity::Luxuries;
    next_kingdom_state.commodity_stores[COMMODITY] = changer(next_kingdom_state.commodity_stores[COMMODITY]);

    (next_turn_state, next_kingdom_state)
}

fn set_ore(turn_state: &TurnState, kingdom_state: &KingdomState, changer: Box<dyn FnOnce(i8) -> i8>) -> (TurnState, KingdomState) {
    let next_turn_state = turn_state.clone();
    let mut next_kingdom_state = kingdom_state.clone();

    const COMMODITY: Commodity = Commodity::Ore;
    next_kingdom_state.commodity_stores[COMMODITY] = changer(next_kingdom_state.commodity_stores[COMMODITY]);

    (next_turn_state, next_kingdom_state)
}

fn set_stone(turn_state: &TurnState, kingdom_state: &KingdomState, changer: Box<dyn FnOnce(i8) -> i8>) -> (TurnState, KingdomState) {
    let next_turn_state = turn_state.clone();
    let mut next_kingdom_state = kingdom_state.clone();

    const COMMODITY: Commodity = Commodity::Stone;
    next_kingdom_state.commodity_stores[COMMODITY] = changer(next_kingdom_state.commodity_stores[COMMODITY]);

    (next_turn_state, next_kingdom_state)
}



fn changer_set(_start: i8, set_to: i8) -> i8 {
    set_to
}

fn changer_change(start: i8, change_by: i8) -> i8 {
    start + change_by
}

fn make_changer(spec: String) -> Result<Box<dyn FnOnce(i8) -> i8>, Error> {
    let num = spec.parse::<i8>()?;
    let changer = match spec.chars().nth(0) {
        Some('-') | Some('+') => changer_change,
        _                     => changer_set,
    };

    let closure = move |start| changer(start, num);
    Ok(Box::new(closure))
}

async fn do_set<F>(
    ctx: Context<'_>,
    change: String,
    update_func: F,
    roll_description: &str,
) -> Result<(), Error>
    where F: FnOnce(&TurnState, &KingdomState, Box<dyn FnOnce(i8) -> i8>) -> (TurnState, KingdomState)
{
    let state_changes = {
        let mut state = ctx.data().tracker.lock().unwrap();
        let changer = make_changer(change)?;
        state.make_update(
            roll_description.to_string(),
            update_func,
            changer,
        )
    };

    let mut text = format!("## {roll_description}");

    for change in &state_changes {
        text.push_str("\n* ");
        text.push_str(change);
    }

    println!("{}", text);
    ctx.reply(text).await?;

    Ok(())
}

#[poise::command(slash_command, prefix_command)]
async fn rp(
    ctx: Context<'_>,
    change: String,
) -> Result<(), Error> {
    do_set(ctx, change, set_rp, "GM set RP").await
}

#[poise::command(slash_command, prefix_command)]
async fn unrest(
    ctx: Context<'_>,
    change: String,
) -> Result<(), Error> {
    do_set(ctx, change, set_unrest, "GM set Unrest").await
}

#[poise::command(slash_command, prefix_command)]
async fn fame(
    ctx: Context<'_>,
    change: String,
) -> Result<(), Error> {
    do_set(ctx, change, set_fame, "GM set Fame").await
}

#[poise::command(slash_command, prefix_command)]
async fn food(
    ctx: Context<'_>,
    change: String,
) -> Result<(), Error> {
    do_set(ctx, change, set_food, "GM set Food").await
}

#[poise::command(slash_command, prefix_command)]
async fn lumber(
    ctx: Context<'_>,
    change: String,
) -> Result<(), Error> {
    do_set(ctx, change, set_lumber, "GM set Lumber").await
}

#[poise::command(slash_command, prefix_command)]
async fn luxuries(
    ctx: Context<'_>,
    change: String,
) -> Result<(), Error> {
    do_set(ctx, change, set_luxuries, "GM set Luxuries").await
}

#[poise::command(slash_command, prefix_command)]
async fn ore(
    ctx: Context<'_>,
    change: String,
) -> Result<(), Error> {
    do_set(ctx, change, set_ore, "GM set Ore").await
}


#[poise::command(slash_command, prefix_command)]
async fn stone(
    ctx: Context<'_>,
    change: String,
) -> Result<(), Error> {
    do_set(ctx, change, set_stone, "GM set Stone").await
}

fn discharge_requirement(turn_state: &TurnState, kingdom_state: &KingdomState, changer: Box<dyn FnOnce(i8) -> i8>) -> (TurnState, KingdomState) {
    let mut next_turn_state = turn_state.clone();
    let next_kingdom_state = kingdom_state.clone();

    let index = changer(0);
    let index = usize::try_from(index);

    if index.is_err() || index.unwrap() >= next_turn_state.requirements.len() {
        println!("!!! OUCH !!!");
        next_turn_state.requirements.push("That was a bad discharge command".to_string());
        return (next_turn_state, next_kingdom_state);
    }

    next_turn_state.requirements.remove(index.unwrap());

    (next_turn_state, next_kingdom_state)
}

fn discharge_bonus(turn_state: &TurnState, kingdom_state: &KingdomState, changer: Box<dyn FnOnce(i8) -> i8>) -> (TurnState, KingdomState) {
    let mut next_turn_state = turn_state.clone();
    let next_kingdom_state = kingdom_state.clone();

    let index = changer(0);
    let index = usize::try_from(index);

    if index.is_err() || index.unwrap() >= next_turn_state.bonuses.len() {
        println!("!!! OUCH !!!");
        next_turn_state.requirements.push("That was a bad discharge command".to_string());
        return (next_turn_state, next_kingdom_state);
    }

    next_turn_state.bonuses.remove(index.unwrap());

    (next_turn_state, next_kingdom_state)
}

#[poise::command(
    prefix_command,
    slash_command,
    subcommands(
        "requirement",
        "bonus",
    ),
    subcommand_required
)]
pub async fn discharge(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}


#[poise::command(slash_command, prefix_command)]
async fn requirement(
    ctx: Context<'_>,
    change: String,
) -> Result<(), Error> {
    do_set(ctx, change, discharge_requirement, "GM discharged Requirement").await
}

#[poise::command(slash_command, prefix_command)]
async fn bonus(
    ctx: Context<'_>,
    change: String,
) -> Result<(), Error> {
    do_set(ctx, change, discharge_bonus, "GM discharged Bonus").await
}
