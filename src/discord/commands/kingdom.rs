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

pub fn create_aryc() -> Kingdom {
    use Attribute::*;
    use Skill::*;
    use TrainingLevel::*;
    let aryc = Kingdom {
        name: "Aryc".to_string(),
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
    ),
    subcommand_required
)]
pub async fn kingdom(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}
