use kingdom::discord::commands::kingdom::kingdom;
use kingdom::discord::commands::roll::roll;
use kingdom::discord::commands::action::act;
use poise::serenity_prelude as serenity;

use kingdom::discord::Data;

#[tokio::main]
async fn main() {
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![roll(), kingdom(), act()],
            ..Default::default()
        })
        .token(std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN"))
        .intents(serenity::GatewayIntents::non_privileged())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        });

    framework.run().await.unwrap();
}
