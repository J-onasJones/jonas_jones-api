use std::io::{self, Write};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::NaiveDateTime;

pub struct Logger;

impl Logger {
    fn print_colored(message: &str, color_code: &str) {
        // ANSI escape codes for color formatting
        const ANSI_RESET: &str = "\x1B[0m";
        const ANSI_DEFAULT: &str = "\x1B[39m";

        let colored_message = format!("{}{}{}\n", color_code, message, ANSI_RESET);

        // Print the colored message to stdout
        print!("{}", colored_message);
        io::stdout().flush().unwrap(); // Ensure the message is immediately printed
        print!("{}", ANSI_DEFAULT); // Reset color to default for subsequent text
    }

    fn log(level: &str, message: &str) {
        let binding = thread::current();
        let thread_name = binding.name().unwrap_or("unnamed");
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("SystemTime before UNIX EPOCH!")
            .as_secs();

        let formatted_time = format!("[{}]", Logger::format_time(timestamp));
        let log_entry = format!(
            "{} [{}/{}] {}",
            formatted_time, thread_name, level, message
        );

        match level {
            "INFO" => println!("{}", log_entry),
            "WARN" => Logger::print_colored(&log_entry, "\x1B[33m"), // Orange
            "ERROR" => Logger::print_colored(&log_entry, "\x1B[31m"), // Red
            "PANIC" => Logger::print_colored(&log_entry, "\x1B[35m"), // Magenta
            _ => println!("{}", log_entry), // Default case (e.g., for custom log levels)
        }
    }

    pub fn format_time(timestamp: u64) -> String {
        let datetime = NaiveDateTime::from_timestamp(timestamp as i64, 0);
        datetime.format("%H:%M:%S").to_string()
    }

    pub fn info(message: &str) {
        Self::log("INFO", message);
    }

    pub fn warn(message: &str) {
        Self::log("WARN", message);
    }

    pub fn error(message: &str) {
        Self::log("ERROR", message);
    }

    pub fn panic(message: &str) {
        Self::log("PANIC", message);
    }
}

fn main() {
    Logger::info("This is an informational log message.");
    Logger::warn("This is a warning log message.");
    Logger::error("This is an error log message.");
}
