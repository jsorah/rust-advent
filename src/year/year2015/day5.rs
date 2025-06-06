use log::info;

pub struct Day5;

#[derive(PartialEq, Debug)]
pub enum NaughtyOrNice {
    Naughty(String),
    Nice(String),
}

const INPUT: &str = include_str!("day5/input");

impl Day5 {
    pub fn run() {
        let mut count = 0;
        for line in INPUT.lines() {
            match Self::naughty_or_nice(line.to_string()) {
                NaughtyOrNice::Nice(_s) => count += 1,
                _ => continue,
            }
        }

        info!("{}", count);
    }

    pub fn naughty_or_nice(s: String) -> NaughtyOrNice {
        let mut prev: Option<char> = Option::None;

        let mut has_double_letter = false;

        let mut vowel_count = 0;
        let vowels = "aeiouAEIOU";

        let prohibited_strings = [('a', 'b'), ('c', 'd'), ('p', 'q'), ('x', 'y')];

        for (_i, c) in s.chars().enumerate() {
            if vowels.contains(c) {
                vowel_count += 1;
            }

            if prev.is_none() {
                prev = Option::Some(c);
                continue;
            }

            if prohibited_strings
                .iter()
                .any(|&(a, b)| a == prev.unwrap() && b == c)
            {
                return NaughtyOrNice::Naughty(s);
            }

            if !has_double_letter && c == prev.unwrap() {
                has_double_letter = true;
            }

            prev = Option::Some(c);
        }

        if vowel_count > 2 && has_double_letter {
            return NaughtyOrNice::Nice(s);
        } else {
            return NaughtyOrNice::Naughty(s);
        }
    }
}
