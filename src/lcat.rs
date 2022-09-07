use crate::colors::*;
use chrono::DateTime;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct LogEntry {
    message: String,
    level: Level,
    #[serde(rename(deserialize = "@timestamp"))]
    timestamp: String,
    logger_name: String,
    stack_trace: Option<String>,
}

#[derive(Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "UPPERCASE")]
pub enum Level {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
}

#[derive(PartialEq, Eq)]
pub enum StackTraceMode {
    Full,
    Skip,
}

pub fn parse_and_format(
    line: &str,
    min_level: &Level,
    stack_trace_mode: &StackTraceMode,
) -> Result<Option<String>, serde_json::error::Error> {
    serde_json::from_str::<LogEntry>(line).map(|log| {
        if &log.level >= min_level {
            Some(format(&log, stack_trace_mode))
        } else {
            None
        }
    })
}

fn format(log: &LogEntry, stack_trace_mode: &StackTraceMode) -> String {
    format!(
        "{} {} {}{}{}",
        time(log),
        level(log),
        logger(log),
        message(log),
        stack_trace(log, stack_trace_mode)
    )
}

fn time(log: &LogEntry) -> String {
    if let Ok(date) = DateTime::parse_from_rfc3339(&log.timestamp) {
        format!("{}{}", TIME_COLOR, date.format("%m-%d %T%.3f"))
    } else {
        format!("{}{}", TIME_COLOR, &log.timestamp)
    }
}

fn level(log: &LogEntry) -> String {
    match log.level {
        Level::Error | Level::Fatal => format!("{}{:?}", ERROR_COLOR, log.level),
        Level::Warn => format!("{}{:?}", WARN_COLOR, log.level),
        Level::Info => format!("{}{:?}", INFO_COLOR, log.level),
        _ => format!("{}{:?}", OTHER_COLOR, log.level),
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
    format!("{}{}", MESSAGE_COLOR, log.message)
}

fn stack_trace(log: &LogEntry, stack_trace_mode: &StackTraceMode) -> String {
    match &log.stack_trace {
        Some(trace) if *stack_trace_mode == StackTraceMode::Full => {
            format!("\n{}{}", STACKTRACE_COLOR, trace)
        }
        _ => "".to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INFO_LOG: &str = r#"
        {
            "message": "test",
            "level": "INFO",
            "@timestamp": "2020-05-04T11:50:24.758+02:00",
            "logger_name": "test.logger.Name"
        }
    "#;

    #[test]
    fn test_parse_and_format() {
        let expected = format!(
            "{}{} {}{} {}{}",
            TIME_COLOR, "05-04 11:50:24.758", INFO_COLOR, "Info Name:", MESSAGE_COLOR, "test"
        );
        assert_eq!(
            expected,
            parse_and_format(INFO_LOG, &Level::Trace, &StackTraceMode::Full)
                .unwrap()
                .unwrap()
        );
    }

    #[test]
    fn test_parse_and_format_filtering() {
        assert!(
            parse_and_format(INFO_LOG, &Level::Warn, &StackTraceMode::Full)
                .unwrap()
                .is_none()
        );
        assert!(parse_and_format(
            &INFO_LOG.replace("INFO", "WARN"),
            &Level::Warn,
            &StackTraceMode::Full
        )
        .unwrap()
        .is_some());

        assert!(parse_and_format(
            &INFO_LOG.replace("INFO", "WARN"),
            &Level::Error,
            &StackTraceMode::Full
        )
        .unwrap()
        .is_none());

        assert!(parse_and_format(
            &INFO_LOG.replace("INFO", "ERROR"),
            &Level::Warn,
            &StackTraceMode::Full
        )
        .unwrap()
        .is_some());
    }

    #[test]
    fn test_parse_and_format_not_json() {
        assert!(parse_and_format("test message", &Level::Trace, &StackTraceMode::Full).is_err());
    }
}
