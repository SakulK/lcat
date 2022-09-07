use std::error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use structopt::StructOpt;
mod colors;
mod lcat;
use lcat::*;

/// Pretty printing logstash-style json logs
#[derive(StructOpt, Debug)]
#[structopt(name = "lcat")]
struct Opt {
    /// show only logs with level ERROR or higher
    #[structopt(short, long)]
    error: bool,

    /// show only logs with level WARN or higher
    #[structopt(short, long)]
    warn: bool,

    /// show only logs with level INFO or higher
    #[structopt(short, long)]
    info: bool,

    /// ignore lines which fail to parse as logstash json (by default they are printed without changes)
    #[structopt(short, long)]
    logstash_only: bool,

    /// disables stacktrace printing
    #[structopt(short, long)]
    no_stacktraces: bool,

    /// path to file with logs
    #[structopt(name = "FILE", parse(from_os_str))]
    file: Option<PathBuf>,
}

impl Opt {
    fn min_level(&self) -> Level {
        if self.warn {
            Level::Warn
        } else if self.error {
            Level::Error
        } else if self.info {
            Level::Info
        } else {
            Level::Trace
        }
    }

    fn stack_trace_mode(&self) -> StackTraceMode {
        if self.no_stacktraces {
            StackTraceMode::Skip
        } else {
            StackTraceMode::Full
        }
    }

    fn reader(&self) -> Result<Box<dyn BufRead>, std::io::Error> {
        if let Some(file) = &self.file {
            Ok(Box::new(BufReader::new(File::open(file)?)))
        } else {
            Ok(Box::new(BufReader::new(std::io::stdin())))
        }
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let opt = Opt::from_args();
    let min_level = opt.min_level();
    let stack_trace_mode = opt.stack_trace_mode();
    for line in opt.reader()?.lines() {
        let line = line?;
        match parse_and_format(&line, &min_level, &stack_trace_mode) {
            Ok(Some(log)) => println!("{}", log),
            Err(_) if !opt.logstash_only => println!("{}", line),
            _ => (),
        }
    }
    Ok(())
}
