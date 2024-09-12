#![crate_type = "lib"]
#![crate_name = "logger"]

use colored::Colorize;
use pad::{PadStr, Alignment};
use chrono::prelude::*;
use std::backtrace;
use std::io::{self, Write};

enum LogLevel {
    Fatal,
    Error,
    Warn,
    Info,
    Debug,
    Silly
}

pub struct Logger {
    depth: i32,
    colors: bool,
    max_array_length: i32,
    break_length: i32,
    compact: i32
}

#[derive(Debug)]
pub struct Options {
    pub depth: usize,
    pub colors: bool,
    pub max_array_length: usize,
    pub break_length: usize,
    pub compact: usize
}

struct Connectors {
    single_line: &'static str,
    start_line: &'static str,
    line: &'static str,
    end_line: &'static str,
}

impl Default for Connectors {
    fn default() -> Self {
        Connectors {
            single_line: "▪",
            start_line: "┏",
            line: "┃",
            end_line: "┗",
        }
    }
}

impl Logger {
    pub fn new() -> Self {
        Logger {
            depth: 3,
            colors: true,
            max_array_length: 120,
            break_length: 60,
            compact: 0
        }
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
        let month = (now.to_utc().month() + 1).to_string().pad_to_width_with_alignment(2, pad::Alignment::Left);
        let day = now.to_utc().day().to_string().pad_to_width_with_alignment(2, pad::Alignment::Left);
        let hour = now.to_utc().hour().to_string().pad_to_width_with_alignment(2, pad::Alignment::Left);
        let minute = now.to_utc().minute().to_string().pad_to_width_with_alignment(2, pad::Alignment::Left);
        let second = now.to_utc().second().to_string().pad_to_width_with_alignment(2, pad::Alignment::Left);

        let time_format = format!("[{}-{}-{} {}:{}:{}]", year, month, day, hour, minute, second);
        return time_format.dimmed().to_string()
    }

    fn get_calle(&self) -> String {
        let backtrace = backtrace::Backtrace::capture();
        let backtrace_str = format!("{:?}", backtrace);
        let lines: Vec<&str> = backtrace_str.lines().collect();

        if lines.len() < 4 {
            return "".to_string();
        }

        let calle = lines[3];

        format!("{}", calle.italic())

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

    fn write<T: std::fmt::Debug>(&self, message: &str, tag: &str, level: LogLevel, at: bool, object: &Option<T>) -> io::Result<()> {
        let message = message.to_string();
        let tag = tag.to_string();
        let connectors = &Connectors::default();
        let color = self.get_colour(&level);
        let timestamp = self.timestamp();
        let timestamp_padding = " ".repeat(21);
        let level_tag = self.get_tag(&level);
        let dim_level_tag = " ".repeat(6);
        let domain_tag = format!("[{}]", tag.color(color));
        let main_message = message.color(color);
    
        let mut log = format!(
            "{} {} {} {} {}\n",
            timestamp, level_tag, connectors.start_line, domain_tag, main_message
        );
        
        let meta_lines = match object {
            Some(object) => {
                let object_str = format!("{:#?}", object);
                object_str.lines().map(|line| line.to_string()).collect()
            }
            None => vec![],
        };

        if at {
            let callee = self.get_calle().dimmed();
            let connector = if meta_lines.is_empty() {
                connectors.end_line
            } else {
                connectors.line
            };

            if !callee.is_empty() {
                log.push_str(&format!(
                    "{} {} {} {}\n",
                    timestamp_padding, dim_level_tag, connector, callee
                ));
            }
        }
    
        for (i, line) in meta_lines.iter().enumerate() {
            let connector = if i == meta_lines.len() - 1 {
                connectors.end_line
            } else {
                connectors.line
            };
            let line_number = format!("[{}]", i + 1).dimmed();
            let meta_line = if i > 2 { line.dimmed().to_string() } else { line.to_string() };
            log.push_str(&format!(
                "{} {} {} {} {}",
                timestamp_padding, dim_level_tag, connector, line_number, meta_line
            ));
        }
    
        let stdout = std::io::stdout();
        let mut handle = stdout.lock();
        writeln!(handle, "{}", log)?;
    
        Ok(())
    }

    fn write_single(&self, message: &str, tag: &str, level: LogLevel) -> io::Result<()> {
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
        writeln!(handle, "{}", log)?;
    
        Ok(())

    }

    pub fn silly<T: std::fmt::Debug>(&self, message: &str, tag: &str, at: bool, object: &Option<T>) -> io::Result<()> {
        self.write(message, tag, LogLevel::Silly, at, object)
    }

    pub fn silly_single(&self, message: &str, tag: &str) -> io::Result<()> {
        self.write_single(message, tag, LogLevel::Silly)
    }

    pub fn debug<T: std::fmt::Debug>(&self, message: &str, tag: &str, at: bool, object: &Option<T>) -> io::Result<()> {
        self.write(message, tag, LogLevel::Debug, at, object)
    }

    pub fn debug_single(&self, message: &str, tag: &str) -> io::Result<()> {
        self.write_single(message, tag, LogLevel::Debug)
    }

    pub fn info<T: std::fmt::Debug>(&self, message: &str, tag: &str, at: bool, object: &Option<T>) -> io::Result<()> {
        self.write( message, tag, LogLevel::Info, at, object)
    }

    pub fn info_single(&self, message: &str, tag: &str) -> io::Result<()> {
        self.write_single( message, tag, LogLevel::Info)
    }

    pub fn warn<T: std::fmt::Debug>(&self, message: &str, tag: &str, at: bool, object: &Option<T>) -> io::Result<()> {
        self.write( message, tag, LogLevel::Warn, at, object)
    }

    pub fn warn_single(&self, message: &str, tag: &str) -> io::Result<()> {
        self.write_single( message, tag, LogLevel::Warn)
    }

    pub fn error<T: std::fmt::Debug>(&self, message: &str, tag: &str, at: bool, object: &Option<T>) -> io::Result<()> {
        self.write( message, tag, LogLevel::Error, at, object)
    }

    pub fn error_single(&self, message: &str, tag: &str) -> io::Result<()> {
        self.write_single( message, tag, LogLevel::Error)
    }

    pub fn fatal<T: std::fmt::Debug>(&self, message: &str, tag: &str, at: bool, object: &Option<T>) -> io::Result<()> {
        self.write( message, tag, LogLevel::Fatal, at, object)
    }

    pub fn fatal_single(&self, message: &str, tag: &str) -> io::Result<()> {
        self.write_single( message, tag, LogLevel::Fatal)
    }
}

fn main() {    
    Logger::new();
}