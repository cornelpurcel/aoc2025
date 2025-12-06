use std::env;
use std::error::Error;
use std::fmt;
use std::fs;

#[derive(Debug)]
struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn contains_number(&self, number: u64) -> bool {
        self.start <= number && number <= self.end
    }

    fn get_fresh_ids_count(&self) -> u64 {
        self.end - self.start + 1
    }
}

impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{} {}]", self.start, self.end)
    }
}

fn simplify_ranges(mut ranges: Vec<Range>) -> Vec<Range> {
    ranges.sort_by_key(|r| r.start);
    let mut result: Vec<Range> = Vec::new();
    for range in ranges {
        if let Some(last_simplified_range) = result.last_mut() {
            if last_simplified_range.contains_number(range.start) {
                if range.end > last_simplified_range.end {
                    last_simplified_range.end = range.end;
                }
            } else {
                result.push(range);
            }
        } else {
            result.push(range);
        }
    }

    result
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    println!("reading the input from {file_path}");

    let (ranges, queries) = parse_file(file_path)?;

    let range_count = ranges.len();

    let ranges = simplify_ranges(ranges);
    let simplified_range_count = ranges.len();
    let query_count = queries.len();

    let mut invalid_ids = 0;
    for query in queries {
        for range in &ranges {
            if range.contains_number(query) {
                invalid_ids += 1;
                break;
            }
        }
    }

    println!(
        "analyzed {query_count} queries in {range_count} ranges and found {invalid_ids} invalid ids."
    );

    println!("Simplified from {range_count} to {simplified_range_count} ranges");
    let fresh_ids_count: u128 = ranges.iter().map(|r| r.get_fresh_ids_count() as u128).sum();
    println!("in total there are {fresh_ids_count} fresh ids!");

    Ok(())
}

fn parse_ranges(raw_string: &str) -> Result<Vec<Range>, Box<dyn Error>> {
    let mut result: Vec<Range> = Vec::new();
    for range in raw_string.split_whitespace().map(|x| x.trim()) {
        let bounds: Vec<&str> = range.split('-').collect();
        let start: u64 = bounds[0].parse::<u64>()?;
        let end: u64 = bounds[1].parse::<u64>()?;
        result.push(Range { start, end });
    }
    Ok(result)
}

fn parse_file(file_path: &String) -> Result<(Vec<Range>, Vec<u64>), Box<dyn Error>> {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut part_iterator = contents.split("\n\n");
    let ranges = part_iterator.next().unwrap();
    let queries: Vec<u64> = part_iterator
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let ranges = parse_ranges(ranges)?;

    Ok((ranges, queries))
}
