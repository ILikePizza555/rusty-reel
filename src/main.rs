use poise::{Framework, FrameworkOptions};
use thiserror::Error;

static WISDOM: Vec<&str> = include_str!("wisdom.txt").lines().collect();

struct Data {}

#[derive(Error, Debug)]
enum RustyReelError {
    #[error("unknown error")]
    Unknown,
}

type Context<'a> = poise::Context<'a, Data, RustyReelError>;

/// Dispenses ancient fox wisdom 
#[poise::command(slash_command, prefix_command, user_cooldown = 5, aliases = ["ping"])]
async fn wisdom(ctx: Context<'_>) -> Result<(), RustyReelError> {

}

#[tokio::main]
async fn main() {
    let framework = Framework::builder();
}
