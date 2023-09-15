//use poise::serenity_prelude as serenity;

use crate::discord::Context;
use crate::discord::Error;
use crate::spec::skills::Skill;
use futures::{Stream, StreamExt};

async fn autocomplete_name<'a>(
    _ctx: Context<'_>,
    partial: &'a str,
) -> impl Stream<Item = String> + 'a {
    let matching_skills = Skill::autocomplete_matching(partial);
    futures::stream::iter(matching_skills)
        .filter(move |name| futures::future::ready(name.starts_with(partial)))
        .map(|name| name.to_string())
}

/// Roll a skill
///
/// Some skill to roll
///
/// Blah blah
#[poise::command(slash_command, prefix_command)]
pub async fn roll(
    ctx: Context<'_>,
    #[description = "Who to greet"]
    #[autocomplete = "autocomplete_name"]
    skill: Option<String>,
) -> Result<(), Error> {
    let response = format!("rolling **{}** again!", skill.unwrap_or_else(|| "??".to_string()));
    ctx.say(response).await?;
    Ok(())
}
