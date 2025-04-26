mod year2015;

use crate::year::year2015::Year2015;

pub enum Year {
    Y2015
}

impl Year {
    pub fn run(&self, day_of_year: u8) {
        match self {
            Year::Y2015 => Year2015::run_day(day_of_year),
        }
    }
}

impl TryFrom<u16> for Year {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            2015 => Ok(Year::Y2015),
            _ => Err(()),
        }
    }
}