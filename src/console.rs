//!
//! THe core log logic
//! May or may not be thread safe
//&!
//! HanishKVC, 2022
//!

pub struct ConsoleLogger {
}

impl ConsoleLogger {
    pub fn new() -> ConsoleLogger {
        ConsoleLogger {  }
    }
}

impl crate::Logger for ConsoleLogger {
    fn log_normal(&self, msg: &str) {
        println!("{}", msg);
    }

    fn log_error(&self, msg: &str) {
        eprintln!("{}", msg);
    }

    fn log_debug(&self, msg: &str) {
        eprintln!("{}", msg);
    }
}