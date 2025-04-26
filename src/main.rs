mod year;

use clap::Parser;
use year::Year;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 1)]
    day: u8,

    #[arg(short, long, default_value_t = 2015)]
    year: u16,
}

fn main() {
    let args = Args::parse();

    match Year::try_from(args.year) {
        Ok(year) => year.run(args.day),
        Err(_) => println!("Unknown year")
    }
}
