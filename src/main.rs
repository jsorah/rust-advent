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
    #[clap(name = "2015")]
    Y2015 {
        #[command(subcommand)]
        command: Y2015Command
    }
}

#[derive(Subcommand, Debug)]
enum Y2015Command {
    #[clap(name = "1")]
    Day1 {
    },
    #[clap(name = "3")]
    Day3 {
    },
    #[clap(name = "4")]
    Day4 {
        #[arg(long)]
        threaded: bool,

        #[arg(long, default_value = "000000")]
        prefix: String,
    },
    #[clap(name = "5")]
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
