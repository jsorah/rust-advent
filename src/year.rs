pub mod year2015;
use crate::year::year2015::Year2015;

use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum YearCommand {
    #[clap(name = "2015")]
    Y2015 {
        #[command(subcommand)]
        command: Y2015Command,
    },
}

#[derive(Subcommand, Debug)]
pub enum Y2015Command {
    #[clap(name = "1")]
    Day1 {},
    #[clap(name = "3")]
    Day3 {},
    #[clap(name = "4")]
    Day4 {
        #[arg(long)]
        threaded: bool,

        #[arg(long, default_value = "000000")]
        prefix: String,
    },
    #[clap(name = "5")]
    Day5 {},
}

pub enum Year {
    Y2015(Y2015Command),
}

impl Year {
    pub fn run(&self) {
        match self {
            Year::Y2015(cmd) => Year2015::run_day(cmd),
        }
    }
}

impl TryFrom<YearCommand> for Year {
    // this still feels really complicated
    type Error = ();
    fn try_from(value: YearCommand) -> Result<Self, Self::Error> {
        match value {
            YearCommand::Y2015 { command } => Ok(Year::Y2015(command)),
            _ => Err(()),
        }
    }
}
