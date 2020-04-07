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
}

impl Drop for StopWatch {
    fn drop(&mut self) {
        println!("\"{}\"\t{}", self.associated_name, self.time_started.elapsed().as_nanos());
    }
}

#[macro_export]
macro_rules! with_time_measuring {
    {
        named_as: $sw_name:literal measures: $block:block
    } => {
        let _sw = StopWatch::start($sw_name);
        $block
    }
}

#[cfg(test)]
mod test {
    use crate::stop_watch::StopWatch;
    #[test]
    fn test_measure_time() {
        with_time_measuring! (
            named_as: "wow!" measures: {
                let x = 1;
                let y = 1;
                x + y
            }
        );
    }
}