use std::sync::Mutex;

use kingdom::discord::commands::{kingdom::kingdom, gm::gm};
use kingdom::discord::commands::roll::roll;
use kingdom::discord::commands::action::act;
use kingdom::tracker::OverallState;
use poise::{serenity_prelude as serenity, PrefixFrameworkOptions};

use kingdom::discord::Data;

#[tokio::main]
async fn main() {
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                roll(),
                kingdom(),
                act(),
                gm(),
            ],
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("!".to_string()),
                execute_self_messages: false,
                ..PrefixFrameworkOptions::default()
            },
            ..Default::default()
        })
        .token(std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN"))
        .intents(
            serenity::GatewayIntents::non_privileged()
            | serenity::GatewayIntents::MESSAGE_CONTENT
        )
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    tracker: Mutex::new(OverallState::new())
                })
            })
        });

    framework.run().await.unwrap();
}
