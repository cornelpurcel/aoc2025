use std::env;
use std::error::Error;
use std::fmt;
use std::fs;
use std::num::ParseIntError;

#[derive(Debug)]
struct BatteryBank {
    batteries: String,
}

fn get_highest_joltage(battery: &str, available_digits: u8) -> Result<u64, ParseIntError> {
    let total_size = battery.len() as u8;

    let mut result = String::new();
    let mut possible_digits_start = 0;
    let mut available_digits = available_digits;

    while available_digits > 0 {
        let candidate_digits_index_start: usize = possible_digits_start;
        let candidate_digits_index_end: usize = (total_size - available_digits + 1).into();

        let candidate_digits = &battery[candidate_digits_index_start..candidate_digits_index_end];

        if candidate_digits_index_start == candidate_digits_index_end {
            // the rest of the string is definitely part of our final number!
            result.push_str(candidate_digits);
            break;
        }

        let (biggest_candidate_index, biggest_candidate_digit) = get_biggest_char(candidate_digits);
        result.push(biggest_candidate_digit);
        possible_digits_start = possible_digits_start + biggest_candidate_index + 1;

        available_digits -= 1;
    }

    result.parse::<u64>()
}

fn get_biggest_char(chars: &str) -> (usize, char) {
    let mut digit = '0';
    let mut index = 0;
    for (i, character) in chars.char_indices() {
        if character > digit {
            digit = character;
            index = i;
        }
    }
    (index, digit)
}

impl fmt::Display for BatteryBank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.batteries)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let number_length: u8 = args[2].parse()?;

    println!("reading the battery banks from {file_path}");

    let battery_banks = parse_file(file_path)?;

    let size = battery_banks.len();

    let mut joltages: Vec<u64> = Vec::new();
    for battery_bank in battery_banks {
        let highest_joltage = get_highest_joltage(&battery_bank.batteries, number_length)?;
        joltages.push(highest_joltage);
    }

    let joltage_sum: u64 = joltages.iter().map(|joltage| *joltage as u64).sum();

    println!("analyzed {size} battery banks and sum of highest joltages is {joltage_sum}");
    Ok(())
}

fn parse_file(file_path: &String) -> Result<Vec<BatteryBank>, Box<dyn Error>> {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    Ok(contents
        .split_whitespace()
        .map(|line| BatteryBank {
            batteries: line.to_string(),
        })
        .collect::<Vec<BatteryBank>>())
}
