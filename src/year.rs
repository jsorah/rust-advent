mod year2015;
use crate::{year::year2015::Year2015, Y2015Command, YearCommand};

pub enum Year {
    Y2015(Y2015Command)
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
            YearCommand::Y2015 {command} => Ok(Year::Y2015(command)),
            _ => Err(()),
        }
    }
}