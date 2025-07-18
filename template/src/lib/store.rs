use crate::Action;

#[derive(Default)]
pub struct Store {
    counter: i32,
    running: bool,
}
impl Store {
    pub fn new() -> Self {
        Self {
            counter: 0,
            running: true,
        }
    }
    pub fn update(&mut self, action: Action) {
        match action {
            Action::Increment => self.counter += 1,
            Action::Decrement => self.counter -= 1,
            Action::Quit => self.running = false,
        }
    }
    pub fn running(&self) -> bool {
        self.running
    }
    pub fn counter(&self) -> i32 {
        self.counter
    }
}
