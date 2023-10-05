pub mod spec;
pub mod rolls;
pub mod turns;
pub mod state;
pub mod actions;
pub mod tracker;
pub mod diff_utils;

pub mod discord;

pub trait Markdownable {
    fn to_markdown(&self) -> String;
}

impl Markdownable for String {
    fn to_markdown(&self) -> String {
        self.clone()
    }
}
