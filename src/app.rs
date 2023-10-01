pub type AppResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct App {
    pub running: bool,
}

impl App {
    pub fn new() -> Self {
        Self { running: true }
    }

    pub fn quit(&mut self) {
        self.running = false;
    }
}
