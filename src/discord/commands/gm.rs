use std::{fs::{OpenOptions, File}, io::BufReader};
use poise;
use crate::{discord::{commands::action, Context, Error}, rolls::{bonus, roll_context::RollType, roll_result::{DegreeOfSuccess, DieRoll, NaturalRoll, RollResult, TotalRoll, DC}}, state::{Commodity, KingdomState}, tracker::OverallState, turns::TurnState};

#[poise::command(
    prefix_command,
    slash_command,
    subcommands(
        "fudge",
        "save",
        "load",
        "rollback",
        "set",
        "discharge",
        "add_bonus",
    ),
    subcommand_required
)]
pub async fn gm(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(prefix_command, slash_command, rename="add-bonus")]
async fn add_bonus(
    ctx: Context<'_>,
    type_: bonus::BonusType,
    applies_to: String,
    applies_until: bonus::AppliesUntil,
    modifier: i8,
    reason: String,
) -> Result<(), Error> {

    println!("add_bonus {applies_to}");

    let applies_to = bonus::AppliesTo::from_string(&applies_to);
    if applies_to.is_none() {
        let _ = ctx.reply("Bad value for applies_to").await;
        println!("  => none");
        return Ok(())
    }
    let applies_to = applies_to.unwrap();
    println!("  => {applies_to:?}");

    let bonus = bonus::Bonus {
        type_: type_,
        applies_to: applies_to,
        applies_until: applies_until,
        modifier: modifier,
        reason: reason.to_string(),
    };

    ////////////////////////////////////////

    action::make_move(
        ctx,
        "GM manually added bonus",
        |_, turn, kingdom_state, _| {
            let roll_result = RollResult {
                die_roll: DieRoll {
                    natural: NaturalRoll(0),
                    total: TotalRoll(0),
                    description: "fake roll".to_string(),
                },
                degree: DegreeOfSuccess::Success,
                dc: DC(0),
            };
            let mut turn_state = turn.clone();
            turn_state.bonuses.push(bonus);
            (
                roll_result,
                turn_state,
                kingdom_state.clone(),
            )
        }
    ).await;

    Ok({})
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
async fn fudge(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(
    prefix_command,
    slash_command,
)]
async fn d4(ctx: Context<'_>, roll: i8) -> Result<(), Error> {
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
async fn d6(
    ctx: Context<'_>,
    roll: i8,
) -> Result<(), Error> {
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
async fn d20(
    ctx: Context<'_>,
    roll: i8,
) -> Result<(), Error> {
    {
        let mut state = ctx.data().tracker.lock().unwrap();
        state.context.d20 = RollType::FixedResult(roll);
    };
    println!("Roll fudged: d20={roll}");
    ctx.say("👍").await?;
    Ok(())
}

#[poise::command(
    prefix_command,
    slash_command,
)]
async fn save(
    ctx: Context<'_>,
    filename: String,
) -> Result<(), Error> {
    println!("About to save!");
    {
        let state = ctx.data().tracker.lock().unwrap();
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(filename)?;
        let s2: &OverallState = &state;
        serde_json::to_writer(&file, &s2)?;
    }
    println!("Saved");
    ctx.say("👍").await?;
    Ok(())
}

#[poise::command(
    prefix_command,
    slash_command,
)]
async fn load(
    ctx: Context<'_>,
    filename: String,
) -> Result<(), Error> {
    println!("About to load!");
    {
        let guard = &mut ctx.data().tracker.lock().unwrap();

        let file = File::open(filename)?;
        let reader = BufReader::new(file);
    
        // Read the JSON contents of the file as an instance of `User`.
        let something: OverallState = serde_json::from_reader(reader)?;
        **guard = something;
    }
    println!("Loaded");
    ctx.say("👍").await?;
    Ok(())
}

#[poise::command(
    prefix_command,
    slash_command,
)]
async fn rollback(ctx: Context<'_>) -> Result<(), Error> {
    let changes = {
        let mut state = ctx.data().tracker.lock().unwrap();

        if state.turns.len() < 2 {
            None
        }
        else {
            let (prev, current) = match &state.turns[..] {
                [.., prev, current] => (prev, current),
                _ => panic!("Huh?"),
            };
            let changes = current.diff(prev);
            state.turns.pop();
            Some(changes)
        }
    };

    match changes {
        Some(changes) => {
            let mut text = "## Rolling back the most recent turn\nThe undo has the effect of appling these changes:".to_string();

            for change in &changes {
                text.push_str("\n* ");
                text.push_str(change);
            }

            println!("{}", text);
            ctx.reply(text).await?;
        },
        None => {
            ctx.reply("Cannot pop the only 'turn'").await?;
        }
    }    
    Ok(())
}


#[poise::command(
    prefix_command,
    slash_command,
    subcommands(
        "size",
        "xp",
        "unrest",
        "rp",
        "fame",
        "food",
        "lumber",
        "luxuries",
        "ore",
        "stone",

        "food_income",
        "lumber_income",
        // TODO: luxury
        "ore_income",
        "stone_income",
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

fn set_size(turn_state: &TurnState, kingdom_state: &KingdomState, changer: Box<dyn FnOnce(i8) -> i8>) -> (TurnState, KingdomState) {
    let next_turn_state = turn_state.clone();
    let mut next_kingdom_state = kingdom_state.clone();

    next_kingdom_state.size = changer(next_kingdom_state.size);

    (next_turn_state, next_kingdom_state)
}

fn set_xp(turn_state: &TurnState, kingdom_state: &KingdomState, changer: Box<dyn FnOnce(i16) -> i16>) -> (TurnState, KingdomState) {
    let next_turn_state = turn_state.clone();
    let mut next_kingdom_state = kingdom_state.clone();

    next_kingdom_state.xp = changer(next_kingdom_state.xp);

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



fn set_food_income(turn_state: &TurnState, kingdom_state: &KingdomState, changer: Box<dyn FnOnce(i8) -> i8>) -> (TurnState, KingdomState) {
    let mut next_turn_state = turn_state.clone();
    let next_kingdom_state = kingdom_state.clone();

    const COMMODITY: Commodity = Commodity::Food;
    next_turn_state.commodity_income[COMMODITY] = changer(next_turn_state.commodity_income[COMMODITY]);

    (next_turn_state, next_kingdom_state)
}

fn set_lumber_income(turn_state: &TurnState, kingdom_state: &KingdomState, changer: Box<dyn FnOnce(i8) -> i8>) -> (TurnState, KingdomState) {
    let mut next_turn_state = turn_state.clone();
    let next_kingdom_state = kingdom_state.clone();

    const COMMODITY: Commodity = Commodity::Lumber;
    next_turn_state.commodity_income[COMMODITY] = changer(next_turn_state.commodity_income[COMMODITY]);

    (next_turn_state, next_kingdom_state)
}

fn set_ore_income(turn_state: &TurnState, kingdom_state: &KingdomState, changer: Box<dyn FnOnce(i8) -> i8>) -> (TurnState, KingdomState) {
    let mut next_turn_state = turn_state.clone();
    let next_kingdom_state = kingdom_state.clone();

    const COMMODITY: Commodity = Commodity::Ore;
    next_turn_state.commodity_income[COMMODITY] = changer(next_turn_state.commodity_income[COMMODITY]);

    (next_turn_state, next_kingdom_state)
}

fn set_stone_income(turn_state: &TurnState, kingdom_state: &KingdomState, changer: Box<dyn FnOnce(i8) -> i8>) -> (TurnState, KingdomState) {
    let mut next_turn_state = turn_state.clone();
    let next_kingdom_state = kingdom_state.clone();

    const COMMODITY: Commodity = Commodity::Stone;
    next_turn_state.commodity_income[COMMODITY] = changer(next_turn_state.commodity_income[COMMODITY]);

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

fn changer_set_i16(_start: i16, set_to: i16) -> i16 {
    set_to
}

fn changer_change_i16(start: i16, change_by: i16) -> i16 {
    start + change_by
}

fn make_changer_i16(spec: String) -> Result<Box<dyn FnOnce(i16) -> i16>, Error> {
    let num = spec.parse::<i16>()?;
    let changer = match spec.chars().nth(0) {
        Some('-') | Some('+') => changer_change_i16,
        _                     => changer_set_i16,
    };

    let closure = move |start| changer(start, num);
    Ok(Box::new(closure))
}

pub async fn do_set<F>(
    ctx: Context<'_>,
    change: String,
    update_func: F,
    roll_description: &str,
) -> Result<(), Error>
    where F: FnOnce(&TurnState, &KingdomState, Box<dyn FnOnce(i8) -> i8>) -> (TurnState, KingdomState)
{
    let player = &ctx.author().name;
    let state_changes = {
        let mut state = ctx.data().tracker.lock().unwrap();
        let changer = make_changer(change)?;
        state.make_update(
            player,
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

pub async fn do_set_i16<F>(
    ctx: Context<'_>,
    change: String,
    update_func: F,
    roll_description: &str,
) -> Result<(), Error>
    where F: FnOnce(&TurnState, &KingdomState, Box<dyn FnOnce(i16) -> i16>) -> (TurnState, KingdomState)
{
    let player = &ctx.author().name;
    let state_changes = {
        let mut state = ctx.data().tracker.lock().unwrap();
        let changer = make_changer_i16(change)?;
        state.make_update_i16(
            player,
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
async fn size(
    ctx: Context<'_>,
    change: String,
) -> Result<(), Error> {
    do_set(ctx, change, set_size, "GM set Size").await
}

#[poise::command(slash_command, prefix_command)]
async fn xp(
    ctx: Context<'_>,
    change: String,
) -> Result<(), Error> {
    do_set_i16(ctx, change, set_xp, "GM set XP").await
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
    let ans = do_set(ctx, change, set_unrest, "GM set Unrest").await;
    let _ = ctx.reply("If you have increased unrest because of an event in Safe Harbor, remember to reduce the decrease by 1 because of the Cemetery there!").await;
    ans
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


#[poise::command(
    slash_command,
    prefix_command,
    rename="food-income",
)]
async fn food_income(
    ctx: Context<'_>,
    change: String,
) -> Result<(), Error> {
    do_set(ctx, change, set_food_income, "GM set Food income").await
}

#[poise::command(
    slash_command,
    prefix_command,
    rename="lumber-income",
)]
async fn lumber_income(
    ctx: Context<'_>,
    change: String,
) -> Result<(), Error> {
    do_set(ctx, change, set_lumber_income, "GM set Lumber income").await
}

#[poise::command(
    slash_command,
    prefix_command,
    rename="ore-income",
)]
async fn ore_income(
    ctx: Context<'_>,
    change: String,
) -> Result<(), Error> {
    do_set(ctx, change, set_ore_income, "GM set Ore income").await
}

#[poise::command(
    slash_command,
    prefix_command,
    rename="stone-income",
)]
async fn stone_income(
    ctx: Context<'_>,
    change: String,
) -> Result<(), Error> {
    do_set(ctx, change, set_stone_income, "GM set Stone income").await
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
