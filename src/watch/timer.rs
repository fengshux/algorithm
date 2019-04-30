use std::time::{Duration, Instant};

pub struct Timer {
    t: Instant
}

impl Timer {
    pub fn new() -> Box<Timer>{        
        Box::new(Timer{t: Instant::now()})
    }

    pub fn end(&self) -> u128 {
        // self.t.elapsed().as_millis()
        // self.t.elapsed().as_nanos()
        self.t.elapsed().as_micros()
    }
}
