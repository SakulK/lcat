use chrono::DateTime;
use serde::{Deserialize, Serialize};
use serde_json;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
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

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <filename>", args[0]);
    } else {
        let file = BufReader::new(File::open(&args[1]).unwrap());
        for line in file.lines() {
            print_log_line(line.unwrap()).unwrap();
        }
    }
}

fn print_log_line(line: String) -> Result<(), Error> {
    let log: LogEntry = serde_json::from_str(&line)?;
    println!(
        "{} {} {}{}{}",
        time(&log),
        level(&log),
        logger(&log),
        log.message,
        stack_trace(&log)
    );
    Ok(())
}

fn time(log: &LogEntry) -> String {
    let time_str = DateTime::parse_from_rfc3339(&log.timestamp)
        .unwrap()
        .format("%m-%d %T%.3f");
    format!("{}{}", color::Fg(color::LightBlack), time_str)
}

fn level(log: &LogEntry) -> String {
    match log.level.as_ref() {
        "ERROR" | "FATAL" => format!("{}{}", color::Fg(color::Red), log.level),
        "WARN" => format!("{}{}", color::Fg(color::Yellow), log.level),
        "INFO" => format!("{}{}", color::Fg(color::White), log.level),
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

fn stack_trace(log: &LogEntry) -> String {
    match &log.stack_trace {
        Some(trace) => format!("\n{}{}", color::Fg(color::LightWhite), trace),
        _ => "".to_owned(),
    }
}
