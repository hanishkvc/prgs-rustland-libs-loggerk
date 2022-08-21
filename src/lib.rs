//!
//! Help log messages in programs
//! * into different categories
//! * inturn logging onto console or file
//!
//! A bunch of direct helpers (logi, loge, logd) are provided, which maps to the console by default.
//! * using these direct helpers is not thread safe in a strict sense.
//! * one needs to call init before using any of these.
//!
//! HanishKVC, 2022
//!


trait Logger {
    fn log_info(&self, msg: &str);
    fn log_error(&self, msg: &str);
    fn log_debug(&self, msg: &str);
}

mod console;

static mut THELOGGER: Option<Box<dyn Logger>> = None;

pub fn init() {
    let clogger = console::ConsoleLogger::new();
    unsafe {
        THELOGGER = Some(Box::new(clogger));
    }
}

pub fn logi(msg: &str) {
    unsafe {
        if THELOGGER.is_none() {
            panic!("ERRR:LoggerK:Direct helper logi called before init");
        }
        let oklogger = THELOGGER.as_ref().unwrap();
        oklogger.log_info(msg);
    }
}

pub fn loge(msg: &str) {
    unsafe {
        let oklogger = THELOGGER.as_ref().expect("ERRR:LoggerK:Direct helper loge called before init");
        oklogger.log_error(msg);
    }
}

pub fn logd(msg: &str) {
    unsafe {
        let oklogger = THELOGGER.as_ref().expect("ERRR:LoggerK:Direct helper logd called before init");
        oklogger.log_debug(msg);
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
