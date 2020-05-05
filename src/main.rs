use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use structopt::StructOpt;
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
        } else {
            lcat::Level::TRACE
        }
    }

    fn reader(&self) -> Box<dyn BufRead> {
        if let Some(file) = &self.file {
            Box::new(BufReader::new(File::open(file).unwrap()))
        } else {
            Box::new(BufReader::new(std::io::stdin()))
        }
    }
}

fn main() {
    let opt = Opt::from_args();
    let min_level = opt.min_level();
    for line in opt.reader().lines() {
        if let Some(log) = lcat::parse_and_format(line.unwrap(), &min_level) {
            println!("{}", log);
        }
    }
}
