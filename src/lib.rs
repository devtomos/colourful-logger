#![crate_type = "lib"]

use colored::Colorize;
use pad::{PadStr, Alignment};
use chrono::prelude::*;
use std::io::Write;

enum LogLevel {
    Fatal,
    Error,
    Warn,
    Info,
    Debug,
    Silly
}

pub struct Logger {}

#[warn(dead_code)]
struct Connectors {
    single_line: &'static str,
}

impl Default for Connectors {
    fn default() -> Self {
        Connectors {
            single_line: "▪",
        }
    }
}

impl Logger {
    pub fn new() -> Self {
        Logger {}
    }

    fn get_tag(&self, level: &LogLevel) -> String {
        match level {
            LogLevel::Silly =>  format!("{}", "silly:".pad_to_width_with_alignment(6, Alignment::Left).bright_magenta()),
            LogLevel::Debug =>  format!("{}", "debug:".pad_to_width_with_alignment(6, Alignment::Left).bright_blue()),
            LogLevel::Info =>   format!("{}", "info:".pad_to_width_with_alignment(6, Alignment::Left).bright_green()),
            LogLevel::Warn =>   format!("{}", "warn:".pad_to_width_with_alignment(6, Alignment::Left).bright_yellow()),
            LogLevel::Error =>  format!("{}", "error:".pad_to_width_with_alignment(6, Alignment::Left).bright_red()),
            LogLevel::Fatal =>  format!("{}", "fatal:".pad_to_width_with_alignment(6, Alignment::Left).red()),
        }
    }

    fn timestamp(&self) -> String {
        let now: DateTime<Local> = Local::now();

        let year = now.to_utc().year();
        let month = (now.to_utc().month() + 1).to_string();
        let day = now.to_utc().day().to_string();
        let hour = now.to_utc().hour().to_string();
        let minute = now.to_utc().minute().to_string();
        let second = now.to_utc().second().to_string();

        let time_format = format!("[{}-{}-{} {}:{}:{}]", year, month, day, hour, minute, second);
        return time_format.dimmed().to_string()
    }

    fn get_colour(&self, level: &LogLevel) -> colored::Color {
        match level {
            LogLevel::Silly =>  colored::Color::BrightMagenta,
            LogLevel::Debug =>  colored::Color::BrightBlue,
            LogLevel::Info =>   colored::Color::BrightGreen,
            LogLevel::Warn =>   colored::Color::BrightYellow,
            LogLevel::Error =>  colored::Color::BrightRed,
            LogLevel::Fatal =>  colored::Color::Red,
        }
    }
 
    fn _write(&self, message: &str, tag: &str, level: LogLevel)  {
        let message = message.to_string();
        let tag = tag.to_string();
        let connectors = &Connectors::default();
        let color = self.get_colour(&level);
        let timestamp = self.timestamp();
        let level_tag = self.get_tag(&level);
        let domain_tag = format!("[{}]", tag.color(color));
        let main_message = message.color(color);
        let log = format!(
            "{} {} {} {} {}",
            timestamp, level_tag, connectors.single_line, domain_tag, main_message
        );

        let stdout = std::io::stdout();
        let mut handle = stdout.lock();
        writeln!(handle, "{}", log).unwrap();
    }

    pub fn silly(&self, message: &str, tag: &str)  {
        self._write(message, tag, LogLevel::Silly)
    }

    pub fn debug(&self, message: &str, tag: &str)  {
        self._write(message, tag, LogLevel::Debug)
    }

    pub fn info(&self, message: &str, tag: &str)  {
        self._write( message, tag, LogLevel::Info)
    }

    pub fn warn(&self, message: &str, tag: &str)  {
        self._write( message, tag, LogLevel::Warn)
    }

    pub fn error(&self, message: &str, tag: &str)  {
        self._write( message, tag, LogLevel::Error)
    }

    pub fn fatal(&self, message: &str, tag: &str)  {
        self._write( message, tag, LogLevel::Fatal)
    }
}