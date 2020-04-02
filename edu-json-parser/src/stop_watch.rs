use std::time::Instant;

pub struct StopWatch {
    associated_name: &'static str,
    time_started: Instant
}

impl StopWatch {
    pub fn start(associated_name: &'static str) -> Self {
        StopWatch {
            associated_name,
            time_started: Instant::now()
        }
    }
    pub fn report(&mut self) {
        println!("\"{}\"\t{}", self.associated_name, self.time_started.elapsed().as_nanos());
    }
}