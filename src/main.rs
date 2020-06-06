use std::error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use structopt::StructOpt;
mod colors;
mod lcat;

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

    /// path to file with logs
    #[structopt(name = "FILE", parse(from_os_str))]
    file: Option<PathBuf>,
}

impl Opt {
    fn min_level(&self) -> lcat::Level {
        if self.warn {
            lcat::Level::WARN
        } else if self.error {
            lcat::Level::ERROR
        } else if self.info {
            lcat::Level::INFO
        } else {
            lcat::Level::TRACE
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
    for line in opt.reader()?.lines() {
        let line = line?;
        match lcat::parse_and_format(&line, &min_level) {
            Ok(Some(log)) => println!("{}", log),
            Err(_) if !opt.logstash_only => println!("{}", line),
            _ => (),
        }
    }
    Ok(())
}
