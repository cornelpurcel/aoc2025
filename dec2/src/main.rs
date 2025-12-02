use std::env;
use std::error::Error;
use std::fmt;
use std::fs;

#[derive(Debug)]
struct Range {
    start: u64,
    end: u64,
}

fn is_invalid_id(number: u64) -> bool {
    let number = number.to_string();
    let size = number.len();
    let max_pattern_size = size / 2;

    for pattern_size in 1..=max_pattern_size {
        if size % pattern_size == 0 {
            let times = size / pattern_size;
            if number[..pattern_size].repeat(times) == number {
                println!("looks like {number} is invalid!!");
                return true;
            }
        }
    }
    return false;
}

fn is_invalid_id_simple(number: u64) -> bool {
    let number = number.to_string();

    let size = number.len();
    if size % 2 == 1 {
        return false;
    }

    let pattern_size = size / 2;

    return number[..pattern_size].repeat(2) == number;
}

impl Range {
    fn get_invalid_ids(&self) -> Vec<u64> {
        return (self.start..=self.end)
            .filter(|x| is_invalid_id(*x))
            .collect();
    }
}

impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{} {}]", self.start, self.end)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    println!("reading the input from {file_path}");

    let ranges = parse_file(file_path)?;

    let size = ranges.len();

    let mut invalid_id_sum: u64 = 0;
    let mut invalid_id_count: usize = 0;
    for range in ranges {
        let invalid_ids = range.get_invalid_ids();
        invalid_id_sum += invalid_ids.iter().sum::<u64>();
        invalid_id_count += invalid_ids.len();
    }

    println!(
        "analyzed {size} commands and found {invalid_id_count} invalid ids. Sum: {invalid_id_sum}"
    );
    Ok(())
}

fn parse_file(file_path: &String) -> Result<Vec<Range>, Box<dyn Error>> {
    let mut result: Vec<Range> = Vec::new();
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    for range in contents.split(',').map(|x| x.trim()) {
        let bounds: Vec<&str> = range.split('-').collect();
        let start: u64 = bounds[0].parse::<u64>()?;
        let end: u64 = bounds[1].parse::<u64>()?;
        // let new_range = ;
        result.push(Range { start, end });
    }

    Ok(result)
}
