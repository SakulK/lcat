use chrono::DateTime;
use serde::{Deserialize, Serialize};
use serde_json;
use termion::color;

#[derive(Serialize, Deserialize, Debug)]
struct LogEntry {
    message: String,
    level: String,
    #[serde(rename(deserialize = "@timestamp"))]
    timestamp: String,
    logger_name: String,
    stack_trace: Option<String>,
}

pub fn parse_and_format(line: String) -> String {
    match serde_json::from_str::<LogEntry>(&line) {
        Ok(log) => format!(
            "{} {} {}{}{}",
            time(&log),
            level(&log),
            logger(&log),
            message(&log),
            stack_trace(&log)
        ),
        _ => line,
    }
}

fn time(log: &LogEntry) -> String {
    match DateTime::parse_from_rfc3339(&log.timestamp) {
        Ok(date) => format!(
            "{}{}",
            color::Fg(color::LightBlack),
            date.format("%m-%d %T%.3f")
        ),
        _ => format!("{}{}", color::Fg(color::LightBlack), &log.timestamp),
    }
}

fn level(log: &LogEntry) -> String {
    match log.level.as_ref() {
        "ERROR" | "FATAL" => format!("{}{}", color::Fg(color::Red), log.level),
        "WARN" => format!("{}{}", color::Fg(color::Yellow), log.level),
        "INFO" => format!("{}{}", color::Fg(color::Reset), log.level),
        _ => format!("{}{}", color::Fg(color::Green), log.level),
    }
}

fn logger(log: &LogEntry) -> String {
    let parts: Vec<&str> = log.logger_name.split('.').collect();
    match parts.last() {
        Some(name) => format!("{}: ", name),
        _ => "".to_owned(),
    }
}

fn message(log: &LogEntry) -> String {
    format!("{}{}", color::Fg(color::Reset), log.message)
}

fn stack_trace(log: &LogEntry) -> String {
    match &log.stack_trace {
        Some(trace) => format!("\n{}{}", color::Fg(color::Reset), trace),
        _ => "".to_owned(),
    }
}
