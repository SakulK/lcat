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
pub enum Level {
    TRACE,
    DEBUG,
    INFO,
    WARN,
    ERROR,
    FATAL,
}

pub fn parse_and_format(line: &str, min_level: &Level) -> Result<Option<String>, serde_json::error::Error> {
    serde_json::from_str::<LogEntry>(line).map(|log|
        if &log.level >= min_level {
            Some(format(&log))
        } else {
            None
        }
    )
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
    if let Ok(date) = DateTime::parse_from_rfc3339(&log.timestamp) {
        format!(
            "{}{}",
            color::Fg(color::LightBlack),
            date.format("%m-%d %T%.3f")
        )
    } else {
        format!("{}{}", color::Fg(color::LightBlack), &log.timestamp)
    }
}

fn level(log: &LogEntry) -> String {
    match log.level {
        Level::ERROR | Level::FATAL => format!("{}{:?}", color::Fg(color::Red), log.level),
        Level::WARN => format!("{}{:?}", color::Fg(color::Yellow), log.level),
        Level::INFO => format!("{}{:?}", color::Fg(color::Reset), log.level),
        _ => format!("{}{:?}", color::Fg(color::Green), log.level),
    }
}

fn logger(log: &LogEntry) -> String {
    let parts: Vec<&str> = log.logger_name.split('.').collect();
    if let Some(name) = parts.last() {
        format!("{}: ", name)
    } else {
        "".to_owned()
    }
}

fn message(log: &LogEntry) -> String {
    format!("{}{}", color::Fg(color::Reset), log.message)
}

fn stack_trace(log: &LogEntry) -> String {
    if let Some(trace) = &log.stack_trace {
        format!("\n{}{}", color::Fg(color::Reset), trace)
    } else {
        "".to_owned()
    }
}
