pub struct Day5;

#[derive(PartialEq, Debug)]
pub enum NaughtyOrNice {
    Naughty(String),
    Nice(String),
}

impl Day5 {
    pub fn run() {}

    fn contains_prohibited_string(s: &str) -> bool {
        s.contains("ab") || s.contains("cd") || s.contains("pq") || s.contains("xy")
    }

    pub fn nice_string(s: String) -> NaughtyOrNice {
        // contains at least three vowels
        // contain a double letter
        // does not contain strings ab, cd, pq, or xy
        if Self::contains_prohibited_string(&s) {
            return NaughtyOrNice::Naughty(s);
        }

        NaughtyOrNice::Nice(s)
    }
}
