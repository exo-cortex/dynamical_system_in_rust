use std::time::Instant;
pub struct Timer {
    current_time: Instant,
}

#[allow(dead_code)]
impl Timer {
    pub fn new() -> Self {
        Timer {
            current_time: Instant::now(),
        }
    }

    pub fn reset(&mut self) {
        self.current_time = Instant::now();
    }

    pub fn get_nanoseconds(&self) -> u128 {
        (Instant::now() - self.current_time).as_nanos()
    }

    pub fn get_nanoseconds_reset(&mut self) -> u128 {
        let duration = (Instant::now() - self.current_time).as_nanos();
        self.reset();
        duration
    }
}
