mod day1;
mod day3;
mod day4;

use crate::Y2015Command;

use crate::year::year2015::day1::Day1;
use crate::year::year2015::day3::Day3;
use crate::year::year2015::day4::Day4;

pub struct Year2015;

impl Year2015 {
    pub fn run_day(day: &Y2015Command) {
        match day {
            Y2015Command::Day1 {} => Day1::run(),
            Y2015Command::Day3 {} => Day3::run(),
            Y2015Command::Day4 { threaded, prefix } => {
                if *threaded {
                    println!("Threaded! Prefix: {}", prefix);
                    Day4::run_threaded(prefix);
                } else {
                    println!("Not Threaded :-( Prefix: {}", prefix);
                    Day4::run_single(prefix);
                }
            }
            _ => println!("No code!"),
        }
    }
}
