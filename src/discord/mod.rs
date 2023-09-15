use std::sync::Mutex;

use crate::tracker::OverallState;

pub mod commands;

pub struct Data {
    pub tracker: Mutex<OverallState>,
} // User data, which is stored and accessible in all command invocations
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
