pub mod year;

use clap::command;
use clap::Parser;
use env_logger::Env;
use year::{Year, YearCommand};

use log::error;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    year: YearCommand,
}

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let args = Args::parse();

    match Year::try_from(args.year) {
        Ok(year) => year.run(),
        Err(_) => error!("Unknown year"),
    }
}
