//use poise::serenity_prelude as serenity;

use crate::discord::Context;
use crate::discord::Error;
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
    let response = format!("rolling **{:?}** again!", skill);
    ctx.say(response).await?;
    Ok(())
}
