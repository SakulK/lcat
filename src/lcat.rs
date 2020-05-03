use chrono::DateTime;
use serde::Deserialize;
use serde_json;
use termion::color;

#[derive(Deserialize, Debug)]
struct LogEntry {
    message: String,
    level: Level,
    #[serde(rename(deserialize = "@timestamp"))]
    timestamp: String,
    logger_name: String,
    stack_trace: Option<String>,
}

#[derive(Deserialize, Debug, PartialEq, PartialOrd)]
#[serde(rename_all = "UPPERCASE")]
pub enum Level {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
}

pub fn parse_and_format(line: String, min_level: &Level) -> Option<String> {
    match serde_json::from_str::<LogEntry>(&line) {
        Ok(log) => {
            if &log.level >= min_level {
                Some(format(&log))
            } else {
                None
            }
        },
        _ => Some(line),
    }
}

fn format(log: &LogEntry) -> String {
    format!(
        "{} {} {}{}{}",
        time(&log),
        level(&log),
        logger(&log),
        message(&log),
        stack_trace(&log)
    )
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
    match log.level {
        Level::Error | Level::Fatal => format!("{}{:?}", color::Fg(color::Red), log.level),
        Level::Warn => format!("{}{:?}", color::Fg(color::Yellow), log.level),
        Level::Info => format!("{}{:?}", color::Fg(color::Reset), log.level),
        _ => format!("{}{:?}", color::Fg(color::Green), log.level),
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
