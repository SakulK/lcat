use std::fs::File;
use std::io::{BufRead, BufReader};
mod lcat;
use std::path::PathBuf;
use structopt::StructOpt;

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
        lcat::Level::WARN
    } else if opt.error {
        lcat::Level::ERROR
    } else {
        lcat::Level::TRACE
    };
    for line in reader.lines() {
        if let Some(log) = lcat::parse_and_format(line.unwrap(), &min_level) {
            println!("{}", log);
        }
    }
}
