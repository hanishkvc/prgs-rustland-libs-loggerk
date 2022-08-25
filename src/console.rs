//!
//! THe core log logic
//! May or may not be thread safe
//!
//! HanishKVC, 2022
//!

pub struct ConsoleLogger {
    binfo: bool,
    berror: bool,
    bwarn: bool,
    bdebug: bool,
    bother: bool,
}

impl ConsoleLogger {
    pub fn new() -> ConsoleLogger {
        ConsoleLogger {
            binfo: true,
            berror: true,
            bwarn: true,
            bdebug: true,
            bother: true,
        }
    }
}

impl crate::Logger for ConsoleLogger {

    fn log_info(&self, msg: &str) {
        if self.binfo {
            println!("{}", msg);
        }
    }

    fn log_error(&self, msg: &str) {
        if !self.berror {
            return;
        }
        eprintln!("{}", msg);
    }

    fn log_warn(&self, msg: &str) {
        if !self.bwarn {
            return;
        }
        eprintln!("{}", msg);
    }

    fn log_debug(&self, msg: &str) {
        if !self.bdebug {
            return;
        }
        eprintln!("{}", msg);
    }

    fn config_info(&mut self, enable: bool) {
        self.binfo = enable;
    }

    fn config_error(&mut self, enable: bool) {
        self.berror = enable;
    }

    fn config_warn(&mut self, enable: bool) {
        self.bwarn = enable;
    }

    fn config_debug(&mut self, enable: bool) {
        self.bdebug = enable;
    }

    fn log_other(&self, msg: &str) {
        if !self.bother {
            return;
        }
        eprintln!("{}", msg);
    }

    fn config_other(&mut self, enable: bool) {
        self.bother = enable;
    }

}
