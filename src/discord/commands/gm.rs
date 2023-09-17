use std::{fs::{OpenOptions, File}, io::BufReader};
use poise;
use crate::{discord::{Context, Error}, rolls::roll_context::RollType, tracker::OverallState};

#[poise::command(
    prefix_command,
    slash_command,
    subcommands(
        "fudge",
        "save",
        "load",
        "rollback",
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
async fn d6(ctx: Context<'_>, roll: i8) -> Result<(), Error> {
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
async fn d20(ctx: Context<'_>, roll: i8) -> Result<(), Error> {
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
async fn save(ctx: Context<'_>, filename: String) -> Result<(), Error> {
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
async fn load(ctx: Context<'_>, filename: String) -> Result<(), Error> {
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
