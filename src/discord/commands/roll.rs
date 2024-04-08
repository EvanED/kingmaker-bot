//use poise::serenity_prelude as serenity;

use crate::discord::Context;
use crate::discord::Error;
use crate::rolls::bonus;
use crate::spec::skills::Skill;

/// Roll a skill*
///
/// Some skill to roll
///
/// Blah blah
#[poise::command(slash_command, prefix_command)]
pub async fn roll(
    ctx: Context<'_>,
    skill: Skill,
) -> Result<(), Error> {
    let roll = {
        let state = ctx.data().tracker.lock().unwrap();
        state.kingdom.roll(bonus::KingdomAction::UNSPECIFIED, skill, &state.context)
    };

    let skill_str: &'static str = skill.into();
    let msg = format!("{skill_str}: {}", roll.to_markdown());

    println!("{}", msg);
    ctx.reply(msg).await?;

    Ok(())
}
