//!
//! Help log messages in programs
//! * into different categories
//! * inturn logging onto console or file
//!
//! A bunch of direct helpers (logi, loge, logd, config, ...) are provided,
//! which maps to the console by default.
//! * using these direct helpers is not thread safe in a strict sense.
//! * one needs to call init before using any of these.
//!
//! HanishKVC, 2022
//!


trait Logger {

    fn log_info(&self, msg: &str);
    fn log_error(&self, msg: &str);
    fn log_warn(&self, msg: &str);
    fn log_debug(&self, msg: &str);
    fn log_other(&self, msg: &str);

    //fn config_force_allto_error(&self);

    fn config_info(&mut self, enable: bool);
    fn config_error(&mut self, enable: bool);
    fn config_warn(&mut self, enable: bool);
    fn config_debug(&mut self, enable: bool);
    fn config_other(&mut self, enable: bool);

}

mod console;

static mut THELOGGER: Option<Box<dyn Logger>> = None;

pub fn log_init() {
    let clogger = console::ConsoleLogger::new();
    unsafe {
        THELOGGER = Some(Box::new(clogger));
    }
}

pub fn log_i(msg: &str) {
    unsafe {
        if THELOGGER.is_none() {
            panic!("ERRR:LoggerK:Direct helper logi called before init");
        }
        let oklogger = THELOGGER.as_ref().unwrap();
        oklogger.log_info(msg);
    }
}

pub fn log_e(msg: &str) {
    unsafe {
        let oklogger = THELOGGER.as_ref().expect("ERRR:LoggerK:Direct helper loge called before init");
        oklogger.log_error(msg);
    }
}

pub fn log_w(msg: &str) {
    unsafe {
        let oklogger = THELOGGER.as_ref().expect("ERRR:LoggerK:Direct helper logw called before init");
        oklogger.log_warn(msg);
    }
}

pub fn log_d(msg: &str) {
    unsafe {
        let oklogger = THELOGGER.as_ref().expect("ERRR:LoggerK:Direct helper logd called before init");
        oklogger.log_debug(msg);
    }
}

pub fn log_o(msg: &str) {
    unsafe {
        let oklogger = THELOGGER.as_ref().expect("ERRR:LoggerK:Direct helper logo called before init");
        oklogger.log_other(msg);
    }
}

pub fn log_config(enable_i: bool, enable_e: bool, enable_w: bool, enable_d: bool, enable_o: bool) {
    unsafe {
        let oklogger = THELOGGER.as_mut().expect("ERRR:LoggerK:Direct helper config called before init");
        oklogger.config_info(enable_i);
        oklogger.config_error(enable_e);
        oklogger.config_warn(enable_w);
        oklogger.config_debug(enable_d);
        oklogger.config_other(enable_o);
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
