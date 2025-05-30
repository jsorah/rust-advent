mod year;

use clap::Parser;
use clap::Subcommand;
use year::Year;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {

    #[command(subcommand)]
    year: YearCommand,

}

#[derive(Subcommand, Debug)]
enum YearCommand {
    Y2015 {
        #[command(subcommand)]
        command: Y2015Command
    }
}

#[derive(Subcommand, Debug)]
enum Y2015Command {
    Day1 {
    },
    Day3 {
    },
    Day4 {
        #[arg(long)]
        threaded: bool,

        #[arg(long, default_value = "000000")]
        prefix: String,
    },
    Day5 {
        
    }
}

fn main() {
    let args = Args::parse();

    match Year::try_from(args.year) {
        Ok(year) => year.run(),
        Err(_) => println!("Unknown year")
    }
}
