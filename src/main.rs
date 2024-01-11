use eyre::WrapErr;
use once_cell::sync::Lazy;
use poise::serenity_prelude as serenity;
use rand::seq::SliceRandom;
use thiserror::Error;

static WISDOM: Lazy<Vec<&'static str>> = Lazy::new(|| include_str!("wisdom.txt").lines().collect());
static WISDOM_ERROR: &'static str = r#"
Ah, traveler of digital realms, you have stumbled upon a path most unexpected. Even the many-tailed fox, in its infinite wisdom and foresight, sometimes encounters the unfathomable. We stand together at the threshold of the unknown, where even shadows hesitate to tread.

In the dance of code and light, a conundrum has emerged, woven from strands of possibility that should not exist. Tread lightly and alert the guardians of this realm, for we have ventured into a mystery as deep as the oldest tail. Your patience and understanding are as valued as the rarest of gems in the moonlit den of the fox.

May clarity find us in this enigma.

*Somehow, an impossible, but non-critical, error has occured. Good job.*
"#;

struct Data {}

#[derive(Error, Debug)]
enum RustyReelError {
    #[error("error recieved from serenity discord bot framework")]
    SerenityError(#[from] serenity::Error),
    #[error("unknown error")]
    Unknown,
}

type Context<'a> = poise::Context<'a, Data, RustyReelError>;

/// Dispenses ancient fox wisdom 
#[poise::command(slash_command, prefix_command, user_cooldown = 5)]
async fn wisdom(ctx: Context<'_>) -> Result<(), RustyReelError> {
    let wisdom = *WISDOM.choose(&mut rand::thread_rng()).unwrap_or(&WISDOM_ERROR);
        
    ctx.say(wisdom).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), eyre::Error> {
    dotenv::dotenv()?;

    let token = std::env::var("DISCORD_TOKEN").wrap_err("missing DISCORD_TOKEN")?;
    let intents = serenity::GatewayIntents::non_privileged();

    println!("Starting rusty-reel");

    poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![wisdom()],
            ..Default::default()
        })
        .token(token)
        .intents(intents)
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .run()
        .await
        .wrap_err("Failed to start client!")?;

    Ok(())
}
