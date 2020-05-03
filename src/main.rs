use std::fs::File;
use std::io::{BufRead, BufReader};
mod lcat;
use structopt::StructOpt;
use std::path::PathBuf;

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

    /// path to file with logs
    #[structopt(name = "FILE", parse(from_os_str))]
    file: Option<PathBuf>,
}

fn main() {
    let opt = Opt::from_args();
    let reader: Box<dyn BufRead> = match opt.file {
        Some(file) => Box::new(BufReader::new(File::open(file).unwrap())),
        _ => Box::new(BufReader::new(std::io::stdin())),
    };
    let min_level = if opt.warn {
        lcat::Level::Warn
    } else if opt.error {
        lcat::Level::Error
    } else {
        lcat::Level::Trace
    };
    for line in reader.lines() {
        match lcat::parse_and_format(line.unwrap(), &min_level) {
            Some(log) => println!("{}", log),
            _ => (),
        }
    }
}
