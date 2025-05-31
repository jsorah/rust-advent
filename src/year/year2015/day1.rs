pub struct Day1;
impl Day1 {
    pub fn run() {
        // TODO - add tests
        // TODO - read from file
        // TODO - optionally, read from STDIN
        let line = "(())(((((((())))))))))))))";

        let mut first_position_basement: Option<u128> = None;

        let mut current_floor: i128 = 0;

        let mut read_position: u128 = 0;

        for c in line.as_bytes() {
            read_position += 1;
            match c {
                b')' => {
                    current_floor += 1;
                }
                b'(' => current_floor -= 1,
                _ => continue,
            }

            if current_floor == -1 && first_position_basement.is_none() {
                first_position_basement = Some(read_position);
            }
        }

        println!("{current_floor}");
        match first_position_basement {
            Some(v) => println!("First time in basement - {v}"),
            None => println!("Didn't go in the basement"),
        }
    }
}
