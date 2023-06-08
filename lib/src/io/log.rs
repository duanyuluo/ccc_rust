use colored::Colorize;

use crate::io::debug::DebugLevel;
use crate::io::provider::stdio;
use crate::io::IoWriter;
// use colored::Colorize;

pub struct Logger {
    output_handle: Box<dyn IoWriter>,
    trigger_level: DebugLevel,
}

impl Logger {
    pub fn create_logger(trigger_level: DebugLevel) -> Logger {
        let output_handle = stdio::StdIoProvider::new_writer(true).unwrap();
        Logger {
            output_handle,
            trigger_level,
        }
    }
    fn format_log(lvl: DebugLevel, msg: &str) -> String {
        match lvl {
            DebugLevel::Info => format!("{}{}", "[INF]=>", Logger::format_msg(msg)),
            DebugLevel::Warn => format!("{}{}", "[WAR]=>".yellow().bold(), Logger::format_msg(msg)),
            DebugLevel::Error => format!("{}{}", "[ERR]=>".red().bold(), Logger::format_msg(msg)),
            DebugLevel::Answer => format!("{}", Logger::format_msg(msg).black().on_blue()),
        }
    }
    fn format_msg(msg: &str) -> String {
        // TODO: replace [[]] to colored string.
        msg.to_string()
    }
    pub fn log(&mut self, lvl: DebugLevel, msg: &str) {
        if self.trigger_level.can_trigger(&lvl) {
            let formatted_msg = Logger::format_log(lvl, msg);
            self.output_handle.write_line(formatted_msg.as_str());
        }
    }
    pub fn info(&mut self, msg: &str) {
        self.log(DebugLevel::Info, msg)
    }
    pub fn warn(&mut self, msg: &str) {
        self.log(DebugLevel::Warn, msg)
    }
    pub fn err(&mut self, msg: &str) {
        self.log(DebugLevel::Error, msg)
    }
    pub fn answer(&mut self, msg: &str) {
        self.log(DebugLevel::Answer, msg)
    }
}

#[cfg(test)]
mod test {
    use colored::Colorize;

    use super::*;

    #[test]
    fn colorize_output() {
        let mut logger = Logger::create_logger(DebugLevel::Info);
        logger.log(
            DebugLevel::Warn,
            format!(
                "This is a log.\n->{}\n->{}",
                "highlight".bright_blue().on_yellow(),
                "error".black().on_red(),
            )
            .as_str(),
        );
    }

    #[test]
    fn all_types_log() {
        let mut logger = Logger::create_logger(DebugLevel::Info);
        logger.log(DebugLevel::Info, "this is information.");
        logger.warn("this is warning.");
        logger.err("this is error.");
        logger.answer("this is answer.");
    }
}
