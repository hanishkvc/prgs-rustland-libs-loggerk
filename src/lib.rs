//!
//! Help log messages in programs
//! * into different categories
//! * inturn logging onto console or file
//!
//! HanishKVC, 2022
//!


trait Logger {
    fn log_normal(&self, msg: &str);
    fn log_error(&self, msg: &str);
    fn log_debug(&self, msg: &str);
}

mod console;



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
