pub mod spec;
pub mod roll_context;
pub mod roll_result;

#[derive(Debug, Default)]
pub struct Cat {
    pub hungry: bool,
}

impl Cat {
    pub fn feed(&mut self) {
        self.hungry = false;
    }
}
