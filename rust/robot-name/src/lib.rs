use std::sync::atomic::{AtomicUsize, Ordering};

pub struct Robot {
    name: String,
}

impl Robot {
    pub fn new() -> Self {
        Robot {
            name: Robot::generate_name(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset_name(&mut self) {
        self.name = Robot::generate_name();
    }

    fn generate_name() -> String {
        static CURSOR: AtomicUsize = AtomicUsize::new(0);
        let n = CURSOR.fetch_add(1, Ordering::Relaxed);
        format!(
            "{}{}{:03}",
            char::from_u32(n as u32 / 1000 / 26 + 65).unwrap(),
            char::from_u32(n as u32 / 1000 % 26 + 65).unwrap(),
            n % 1000
        )
    }
}
