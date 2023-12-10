use std::{
    fmt::{self, Display},
    path::PathBuf,
    str::FromStr,
};

pub const DEFAULT_INPUT_FILE: &str = "input.txt";
pub const RADIX: u32 = 10u32;

pub type Inner = u64;

pub struct CalibrationValue(pub Inner);

impl FromStr for CalibrationValue {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first, last): (char, char) = (
            s.chars()
                .nth(
                    s.find(|c: char| c.is_digit(RADIX))
                        .ok_or("No digits".to_string())?,
                )
                .ok_or("No digits")?,
            s.chars()
                .nth(
                    s.rfind(|c: char| c.is_digit(RADIX))
                        .ok_or("No digits".to_string())?,
                )
                .ok_or("No digits")?,
        );

        Ok(Self(
            [first, last]
                .iter()
                .collect::<String>()
                .parse::<Inner>()
                .map_err(|_e| "Invalid integer")?,
        ))
    }
}

impl Display for CalibrationValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub fn main() {
    let input_file: PathBuf = match std::env::args().nth(1) {
        Some(t) => t.into(),
        None => DEFAULT_INPUT_FILE.into(),
    };

    println!(
        "{}",
        std::fs::read_to_string(input_file)
            .expect("unable to read file")
            .lines()
            .map(|line| line.parse::<CalibrationValue>().unwrap().0)
            .sum::<Inner>()
    );
}
